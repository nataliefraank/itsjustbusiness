// use bevy::input::keyboard::KeyCode;
use bevy::{prelude::*, transform};
// use bevy::window::PrimaryWindow;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
struct Person;

fn main() {
    App::new()
        // Add Bevy default plugins
        .add_plugins(DefaultPlugins)
        // Add bevy_ecs_tilemap plugin
        .add_plugins(TilemapPlugin)
        // Add bevy_ecs_tiled plugin
        .add_plugins(TiledMapPlugin::default())
        // Add our setup system to spawn assets
        .add_systems(Startup, load_sprite)
        // Add the startup system for the map
        .add_systems(Startup, load_tilemap)
        // .add_startup_system(load_tilemap)
        .run();
}

pub fn load_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a sprite with the correct texture path
    commands.spawn(SpriteBundle {
        texture: asset_server.load("janitor-v1.png"),
        transform: Transform::from_xyz(30.0, 3.0, 3.0),
        ..default()
    });
}

pub fn load_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Load the map: ensure any tile / tileset paths are relative to assets/ folder
    let map_handle: Handle<TiledMap> = asset_server.load("tilemap_level1.tmx");

    // Spawn the map with default options
    commands.spawn(TiledMapHandle(map_handle));
    
    let map_handle = Transform::from_xyz(5.0, 6.0, 7.0);
}
