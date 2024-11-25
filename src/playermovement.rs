use bevy::input::common_conditions::{self, *};
use bevy::prelude::*;

#[derive(Component)]
struct Player {
    // position: Point,
    // sprite: Rect,
    // speed: i32,
}

fn main() {
    // let mut app = App::new();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// System for handling player movement
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    // Normalize direction to ensure consistent movement and scale with speed
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let speed = 5.0;
    let delta_time = 1.0 / 60.0; // Assuming a fixed frame rate for simplicity
    player_transform.translation += direction * speed * delta_time;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("janitor-v1.png"),
            transform: Transform::from_xyz(30.0, 3.0, 0.0),
            ..default()
        })
        .insert(Player {})
        .insert(TransformBundle::from(Transform::default()))
        .insert(VisibilityBundle::default());

    commands.spawn((
        Player {},
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
    ));
}
