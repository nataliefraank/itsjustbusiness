use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{FacingDirectionEnum, Movement, Player};

pub fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Player,
        &mut Movement,
        &mut FacingDirection,
        &mut Velocity,
    )>,
) {
    for (_player, movement, mut direction, mut velocity) in query.iter_mut() {
        let mut velocity_delta = Vec2::new(0.0, 0.0);
        let mut x_delta = 0.0;
        let mut y_delta = 0.0;
        // Up
        if keyboard.pressed(KeyCode::W) {
            y_delta += 1.0;
        }
        // Down
        else if keyboard.pressed(KeyCode::S) {
            y_delta -= 1.0;
        }
        // Left
        if keyboard.pressed(KeyCode::A) {
            x_delta -= 1.0;
        }
        // Right
        else if keyboard.pressed(KeyCode::D) {
            x_delta += 1.0;
        }

        velocity_delta.x = x_delta;
        velocity_delta.y = y_delta;

        if velocity_delta != Vec2::ZERO {
            // Normalize
            velocity_delta /= velocity_delta.length();
            velocity_delta *= movement.speed;
        }

        // Update facing direction
        if velocity_delta.x != 0.0 {
            if velocity_delta.x > 0.0 {
                direction.direction = FacingDirectionEnum::Right;
            } else {
                direction.direction = FacingDirectionEnum::Left;
            }
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        velocity.linvel = velocity_delta;
    }
}

// use bevy::a11y::accesskit::Point;
// use bevy::input::common_conditions::{self, *};
// use bevy::prelude::*;

// #[derive(Component)]
// struct Player {
//     position: Point,
//     sprite: Rect,
//     speed: i32,
// }

// fn main() {
//     // let mut app = App::new();
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .run();
// }

// // System for handling player movement
// fn player_movement(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     let mut player_transform = query.single_mut();
//     let mut direction = Vec3::ZERO;

//     if keyboard_input.pressed(KeyCode::KeyW) {
//         direction.y += 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyS) {
//         direction.y -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyA) {
//         direction.x -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) {
//         direction.x += 1.0;
//     }

//     // Normalize direction to ensure consistent movement and scale with speed
//     if direction.length_squared() > 0.0 {
//         direction = direction.normalize();
//     }

//     let speed = 5.0;
//     let delta_time = 1.0 / 60.0; // Assuming a fixed frame rate for simplicity
//     player_transform.translation += direction * speed * delta_time;
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn(SpriteBundle {
//             texture: asset_server.load("janitor-v1.png"),
//             transform: Transform::from_xyz(30.0, 3.0, 0.0),
//             ..default()
//         })
//         .insert(Player {})
//         .insert(TransformBundle::from(Transform::default()))
//         .insert(VisibilityBundle::default());

//     commands.spawn((
//         Player {},
//         Transform::default(),
//         GlobalTransform::default(),
//         Visibility::default(),
//         InheritedVisibility::default(),
//     ));
// }
