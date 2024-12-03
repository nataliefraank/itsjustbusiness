use bevy::{color::palettes::css::PURPLE, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component, Debug)]
struct Player {
    speed: f32,
}

#[derive(Debug)]
enum GameStatus {
    Ongoing,
    Won,
    Lost,
}

#[derive(Debug)]
struct GameState {
    status: GameStatus,
}

#[derive(Debug)]
struct Score {
    value: i32,
}

fn main() {
    App::new()
        // Add Bevy default plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: (140.0, 140.0).into(),
                title: "It's Just Business".to_string(),
                ..default()
            }),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // Add bevy_ecs_tilemap plugin
        .add_plugins(TilemapPlugin)
        // Add bevy_ecs_tiled plugin
        .add_plugins(TiledMapPlugin::default())
        // .add_startup_system(load_tilemap)
        .run();
}

pub fn load_sprite(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("janitor-v1.png"),
        transform: Transform::from_xyz(30.0, 3.0, 3.0),
        ..Default::default()
    });
}

pub fn load_tilemap(commands: &mut Commands, asset_server: &mut Res<AssetServer>) {
    commands.spawn((
        TiledMapHandle(asset_server.load("tilemap_level1.tmx")),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Adjust these values
        GlobalTransform::default(),
    ));
}

/* OLD CODE: */
// Spawn a 2D camera
// commands.spawn(Camera2dBundle::default());

// Load the map: ensure any tile / tileset paths are relative to assets/ folder
// let map_handle: Handle<TiledMap> = asset_server.load("tilemap_level1.tmx");

// Spawn the map with default options
// commands.spawn(TiledMapHandle(map_handle));
