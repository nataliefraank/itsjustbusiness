use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tiled::{TiledMapHandle, TiledMapPlugin};
use bevy_ecs_tilemap::prelude::*;

mod playermovement;

// Resource to store the map's size and tile size.
#[derive(Resource)]
struct MapInfo {
    map_width: f32,
    map_height: f32,
}

fn main() {
    // Create a new application.
    App::new()
        // Add Bevy default plugins.
        .add_plugins(DefaultPlugins)
        // Add TileMap plugin.
        .add_plugins(TilemapPlugin)
        .insert_resource(MapInfo {
            map_width: 30.0,
            map_height: 20.0,
        })
        // Add setup system.
        .add_systems(Startup, setup)
        // Add camera system.
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, scale_tilemap_to_screen)
        // Add bevy_ecs_tilemap plugin
        .add_plugins(TiledMapPlugin::default())
        .run();
}

// Loads tilemap and janitor sprite.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    // Load the tilemap
    commands.spawn((
        TiledMapHandle(asset_server.load("tilemap_level1.tmx")),
        Transform::default(),
        GlobalTransform::default(),
    ));

    // The tilemap is 20x30 tiles, each 24x24 pixels.
    let map_tile_width = 30.0;
    let map_tile_height = 20.0;
    let tile_size = 24.0;

    let map_width = map_tile_width * tile_size;
    let map_height = map_tile_height * tile_size;

    // Store the map info in a resource
    commands.insert_resource(MapInfo {
        map_width,
        map_height,
    });

    // Spawn the janitor sprite
    let janitor_texture: Handle<Image> = asset_server.load("janitor-v1.png");
    commands.spawn(SpriteBundle {
        texture: janitor_texture,
        transform: Transform {
            translation: Vec3::new(360.0, 410.0, 1.0), // Adjust based on your layout
            ..Default::default()
        },
        ..Default::default()
    });

    info!("Setup complete. Map size: {}x{}", map_width, map_height);
}

fn spawn_camera(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(350.0, 225.0, 1.0)), // Adjust Z for depth
        // transform: Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    });
}

fn scale_tilemap_to_screen(
    mut query: Query<&mut Transform, With<TiledMapHandle>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    map_info: Res<MapInfo>,
) {
    let window = primary_window.single();
    let window_width = window.width();
    let window_height = window.height();

    // Calculate the scale to fit the map to the screen
    let scale_x = window_width / map_info.map_width;
    let scale_y = window_height / map_info.map_height;

    // Choose the smaller scale to ensure the entire map fits within the window
    let scale = scale_x.min(scale_y);

    for mut transform in query.iter_mut() {
        // Apply scaling
        transform.scale = Vec3::splat(scale);

        // Center the map on the screen
        transform.translation = Vec3::new(
            (window_width - map_info.map_width * scale) / 2.0,
            (window_height - map_info.map_height * scale) / 2.0,
            0.0,
        );
    }

    info!(
        "Window size: {}x{}, Map size: {}x{}, Scale: {}",
        window_width, window_height, map_info.map_width, map_info.map_height, scale
    );
}
