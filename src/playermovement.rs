use bevy::{a11y::accesskit::Point, prelude::*};


#[derive(Component)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
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

    // Move the player sprite
    let speed = 5.0;
    player_transform.translation += direction * speed;
}