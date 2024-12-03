use bevy::prelude::*;
use bevy_ecs_tiled::{TiledMapHandle, TiledMapPlugin};
use bevy_ecs_tilemap::prelude::*;

mod helper;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        // Add bevy_ecs_tilemap plugin
        .add_plugins(TiledMapPlugin::default())
        .run();
}

// Resource to track asset loading state
#[derive(Resource)]
struct LoadingState {
    tilemap_loaded: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the tilemap handle
    commands.spawn((
        TiledMapHandle(asset_server.load("tilemap_level1.tmx")),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)), // Adjust these values
        GlobalTransform::default(),
    ));

    // Spawn the janitor sprite
    let janitor_texture: Handle<Image> = asset_server.load("janitor-v1.png");
    commands.spawn(SpriteBundle {
        texture: janitor_texture,
        transform: Transform {
            translation: Vec3::new(-320.0, -320.0, 1.0), // Adjust based on your layout
            ..Default::default()
        },
        ..Default::default()
    });

    info!("Setup complete. Waiting for assets to load...");
}

fn spawn_camera(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)), // Adjust Z for depth
        ..Default::default()
    });
}

// //! This example demonstrates how to load and unload maps.

// use bevy::prelude::*;
// use bevy_ecs_tiled::prelude::*;
// use bevy_ecs_tilemap::prelude::*;

// mod helper;
// mod playermovement;

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
// enum MapState {
//     Loaded,
//     #[default]
//     Unloaded,
// }

// fn main() {
//     App::new()
//         // Bevy default plugins
//         .add_plugins(DefaultPlugins)
//         // Examples helper plugin (does not matter for this example)
//         // .add_plugins(helper::HelperPlugin)
//         // bevy_ecs_tilemap and bevy_ecs_tiled main plugins
//         .add_plugins(TilemapPlugin)
//         .add_plugins(TiledMapPlugin::default())
//         // Add our systems and run the app!
//         .init_state::<MapState>()
//         .add_systems(Startup, startup)
//         .add_systems(
//             Update,
//             (
//                 handle_load.run_if(in_state(MapState::Unloaded)),
//                 (handle_unload, handle_reload, handle_respawn).run_if(in_state(MapState::Loaded)),
//             ),
//         )
//         .add_systems(Update, log_transitions)
//         .run();
// }

// fn startup(
//     mut commands: Commands,
//     mut asset_server: Res<AssetServer>,
//     mut next_state: ResMut<NextState<MapState>>,
// ) {
//     helper::load_sprite(&mut commands, &mut asset_server);
//     helper::load_tilemap(&mut commands, &mut asset_server);
//     next_state.set(MapState::Loaded);
// }

// fn handle_load(
//     mut commands: Commands,
//     mut asset_server: Res<AssetServer>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut next_state: ResMut<NextState<MapState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::KeyL) {
//         helper::load_sprite(&mut commands, &mut asset_server);
//         helper::load_tilemap(&mut commands, &mut asset_server);
//         next_state.set(MapState::Loaded);
//     }
// }

// fn handle_reload(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     maps_query: Query<Entity, With<TiledMapMarker>>,
//     mut next_state: ResMut<NextState<MapState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::KeyK) {
//         if let Ok(entity) = maps_query.get_single() {
//             commands
//                 .entity(entity)
//                 .insert(TiledMapHandle(asset_server.load("tilemap_level1.tmx")));
//         } else {
//             warn!("Cannot reload: no map loaded ?");
//         }

//         next_state.set(MapState::Loaded);
//     }
// }

// fn handle_unload(
//     mut commands: Commands,
//     mut maps: ResMut<Assets<TiledMap>>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     maps_query: Query<Entity, With<TiledMapMarker>>,
//     mut next_state: ResMut<NextState<MapState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::KeyU) {
//         // This example shows that the map gets properly unloaded if the
//         // `TiledMap` asset is removed.
//         //
//         // However, typically you would remove the map entity instead.
//         let handles: Vec<_> = maps.iter().map(|(handle, _)| handle).collect();
//         for handle in handles {
//             // This will cause the map to unload.
//             maps.remove(handle);
//         }

//         // Actually remove the entities, so that we can re-add later.
//         // If we don't do this, the entity still exists and the map will not be
//         // reloaded properly.
//         for entity in maps_query.iter() {
//             commands.entity(entity).despawn_recursive();
//         }
//         next_state.set(MapState::Unloaded);
//     } else if keyboard_input.just_pressed(KeyCode::KeyI) {
//         // Just remove the entities directly. This will also unload the map.
//         for entity in maps_query.iter() {
//             commands.entity(entity).despawn_recursive();
//         }
//         next_state.set(MapState::Unloaded);
//     }
// }

// fn handle_respawn(
//     mut commands: Commands,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     maps_query: Query<Entity, With<TiledMapMarker>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::KeyR) {
//         if let Ok(entity) = maps_query.get_single() {
//             commands.entity(entity).insert(RespawnTiledMap);
//         } else {
//             warn!("Cannot respawn: no map loaded ?");
//         }
//     }
// }

// fn log_transitions(mut transitions: EventReader<StateTransitionEvent<MapState>>) {
//     for transition in transitions.read() {
//         info!(
//             "transition: {:?} => {:?}",
//             transition.exited, transition.entered
//         );
//     }
// }
