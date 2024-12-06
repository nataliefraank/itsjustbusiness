// pub mod animation;
// pub mod collision_events;
// pub use animation::*;
// pub use collision_events::*;

use bevy::sprite;
use bevy_rapier2d::{math::Vect, prelude::Velocity};

use crate::*;

#[derive(Component, Default)]
pub struct Player;

// Components - in this module, I keep only systems to spawn entities with their components.
// Systems that run on every tick I keep in the systems module.

#[derive(Component, Default)]
pub struct DeriveZFromY {
    // visual_base_of_image
    px_below_center: f32,
}

impl DeriveZFromY {
    /*
    The default camera is on Z 1000, lets keep that.
    Dividing Y by 100 (the coefficient) and subtracting from the camera (max possible "mirror_base")
    at 1k would allow for a level up to 100k px vertically.
    Lets keep some room just for good measure. Subtracting from a 100 base gives space for up to 10k px.
    If a higher level (px) would be necessary, the coefficient can be also adjusted.
    btw the level and other entities spawn around Z 0-3.
    */
    const MIRROR_BASE: f32 = 100.;
    const COEFFICIENT: f32 = 100.;
    pub fn get(&self, y: f32) -> f32 {
        Self::MIRROR_BASE - (y - self.px_below_center) / Self::COEFFICIENT
    }

    pub fn spawn(mut query: Query<(&mut Transform, &DeriveZFromY), Added<DeriveZFromY>>) {
        for (mut transform, dzfy) in &mut query {
            transform.translation.z = dzfy.get(transform.translation.y);
        }
    }
}

impl From<i32> for DeriveZFromY {
    fn from(value: i32) -> Self {
        Self {
            px_below_center: value as f32,
        }
    }
}

// Systems - in this module, I keep systems that run on every tick.
// Systems for spawning entities and theirs components I keep in components.

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Velocity,
            &mut TextureAtlas,
            // &mut PlayerAnimationState,
            // &mut SpriteSheetAnimation,
        ),
        With<Player>,
    >,
) {
    //if let Ok((mut velocity, mut sprite, mut state, mut animation)) = query.get_single_mut() {
    if let Ok((mut velocity, sprite)) = query.get_single_mut() {
        const SPEED: f32 = 150.;

        let default = Vect::default();
        if velocity.linvel != default {
            velocity.linvel = default;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            velocity.linvel += Vect::new(0., SPEED);
        }
        if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            velocity.linvel += Vect::new(-SPEED, 0.);
            // sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            velocity.linvel += Vect::new(0., -SPEED);
        }
        if keyboard_input.pressed(KeyCode::ControlRight) || keyboard_input.pressed(KeyCode::KeyD) {
            velocity.linvel += Vect::new(SPEED, 0.);
            // sprite.flip_x = false;
        }

        // if velocity.linvel != default {
        //     if *state != PlayerAnimationState::Running {
        //         // start running
        //         *state = PlayerAnimationState::Running;
        //         *animation = PlayerAnimationState::Running.into();
        //     }
        // } else if *state != PlayerAnimationState::Idle {
        //     // stop running
        //     *state = PlayerAnimationState::Idle;
        //     *animation = PlayerAnimationState::Idle.into();
        // }
    }
}

pub fn derive_z_from_y_after_move(
    mut player_query: Query<(&mut Transform, &DeriveZFromY), Changed<Transform>>,
) {
    if let Ok((mut transform, dzfy)) = player_query.get_single_mut() {
        transform.translation.z = dzfy.get(transform.translation.y);
    }
}

pub fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
