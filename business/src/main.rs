use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
struct Person;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("assets/janitor32x48.png"),..default()
    });
}

fn main() {
    App::new()
        // Add Bevy default plugins
        .add_plugins(DefaultPlugins)
        // Add bevy_ecs_tilemap plugin
        .add_plugins(TilemapPlugin)
        // Add bevy_ecs_tiled plugin
        .add_plugins(TiledMapPlugin::default())
        // Add our startup function to the schedule and run the app
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Load the map: ensure any tile / tileset paths are relative to assets/ folder
    let map_handle: Handle<TiledMap> = asset_server.load("tilemap_level1.tmx");

    // Spawn the map with default options
    commands.spawn(TiledMapHandle(map_handle));
}
