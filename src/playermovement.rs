// System for handling player movement
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    // Move the player sprite
    let speed = 5.0;
    player_transform.translation += direction * speed;
}