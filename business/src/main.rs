use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::prelude::*;

// Window and tile sizes
const TILE_SIZE: f32 = 32.0;
const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .run();
}

// Enum for different types of tiles
#[derive(Debug, Clone, Copy)]
enum TileType {
    Grass,
    Water,
}

// Player component to identify the player entity
#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

// 2D map array to store tile types
const TILE_MAP: [[TileType; MAP_WIDTH]; MAP_HEIGHT] = [[TileType::Grass; MAP_WIDTH]; MAP_HEIGHT];

// Setup function to initialize the game and create the tile map
fn setup(mut commands: Commands) {
    // Set up the camera
    commands.spawn(Camera2dBundle::default());

    // Iterate over each tile in the map array and create a sprite for each
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile_type = TILE_MAP[y][x];
            let color: Color = match tile_type {
                TileType::Grass => Color::rgb(0.1, 0.8, 0.1),
                TileType::Water => Color::rgb(0.1, 0.4, 0.8),
            };

            // Spawn a tile sprite
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * TILE_SIZE - (MAP_WIDTH as f32 / 2.0) * TILE_SIZE,
                        y as f32 * TILE_SIZE - (MAP_HEIGHT as f32 / 2.0) * TILE_SIZE,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}



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
