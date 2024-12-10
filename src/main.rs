use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use bevy_ecs_tiled::{TiledMapHandle, TiledMapPlugin};
use bevy_ecs_tilemap::prelude::*;
// use r#move::{derive_z_from_y_after_move, move_camera, move_player};
use core::time;
use std::time::Duration;

use bevy::{
    ecs::{query, world},
    math::vec3,
    prelude::*,
    reflect::utility::GenericTypeCell,
    render::{camera, render_resource::Texture},
    transform::{self, commands},
    utils::tracing::span::Id,
};
use bevy_tweening::*;
use lens::TransformPositionLens;

mod mainmenu;
// mod r#move;
// mod move2;
// mod playermovement;
use crate::mainmenu::MenuPlugin;
// Resource to store the map's size and tile size.

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Settings,
    Playing,
    Paused,
    Exit,
}

#[derive(Resource)]
struct RootEntity(Entity);

fn despawn_state(mut commands: Commands, root: Res<RootEntity>) {
    commands.entity(root.0).despawn_recursive();
}

fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct MapInfo {
    map_width: f32,
    map_height: f32,
}

#[derive(Bundle)]
struct Player {
    position: Position,
    sprite: SpriteBundle,
    speed: Speed,
}

#[derive(Component)]
struct Position {
    position: Vec<f32>,
}

#[derive(Component)]
struct Speed {
    speed: i32,
}

// #[derive(Component)]
struct MyCameraMarker;

fn main() {
    // Create a new application.
    App::default()
        // Add Bevy default plugins.
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_plugins(TweeningPlugin)
        .init_state::<GameState>()
        // Add MenuPlugin
        // Add TileMap plugin.
        .add_plugins((TilemapPlugin, MenuPlugin))
        .insert_resource(MapInfo {
            map_width: 30.0,
            map_height: 20.0,
        })
        .add_systems(Startup, (spawn_entity)) // Add a startup system to spawn the entity
        // Add setup system.
        .add_systems(Startup, setup)
        // Add camera system.
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, scale_tilemap_to_screen)
        .add_systems(Update, keyboard_input)
        // Add bevy_ecs_tilemap plugin
        .add_plugins(TiledMapPlugin::default())
        // .add_systems(Update, spriteMove)
        // .add_systems(
        //     Update,
        //     (
        //         // move_player,
        //         // move_camera,
        //         // derive_z_from_y_after_move,
        //         // collision_events,
        //     ),
        // )
        .run();
}

// Loads tilemap and janitor sprite.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let mut player_sprite_obj = SpriteBundle::default();
    player_sprite_obj.texture = janitor_texture;
    player_sprite_obj.transform = Transform::from_xyz(360.0, 410.0, 1.0);

    commands.spawn(player_sprite_obj);

    // commands.spawn(SpriteBundle {
    //     texture: janitor_texture,
    //     transform: Transform {
    //         translation: Vec3::new(360.0, 410.0, 1.0), // Adjust based on your layout
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    // commands.spawn((
    //     Player {
    //         position: Point { x: 0.0, y: 0.0 },
    //         sprite: Rect {},
    //         speed: 3,
    //     },
    //     Transform::default(),
    //     GlobalTransform::default(),
    //     Visibility::default(),
    //     InheritedVisibility::default(),
    // ));

    info!("Setup complete. Map size: {}x{}", map_width, map_height);
}

fn spawn_camera(mut commands: Commands) {
    // Spawn a 2D camera
    let mut our_camera = Camera2dBundle::default();
    our_camera.transform = Transform::from_xyz(350.0, 225.0, 1.0);
    our_camera.projection.scaling_mode = ScalingMode::FixedVertical(500.0);

    commands.spawn(our_camera);
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

// fn spriteMove(commands: Commands, sprite: SpriteBundle) {
//     // Create a single animation (tween) to move an entity.
//     let tween = Tween::new(
//         // Use a quadratic easing on both endpoints.
//         EaseFunction::QuadraticInOut,
//         // Animation time.
//         Duration::from_secs(1),
//         // The lens gives access to the Transform component of the Entity,
//         // for the Animator to animate it. It also contains the start and
//         // end values respectively associated with the progress ratios 0. and 1.
//         TransformPositionLens {
//             start: Vec3::ZERO,
//             end: Vec3::new(1., 2., -4.),
//         },
//     );

//     commands.spawn((
//         // Spawn an entity to animate the position of.
//         TransformBundle::default(),
//         // Add an Animator component to control and execute the animation.
//         Animator::new(tween),
//     ));
// }

#[derive(Resource)]
struct PosVar {
    pos_vec: Vec3,
    id: Entity,
    timer: Timer,
    in_anim: bool,
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform>,
    mut local: ResMut<PosVar>,
    mut commands: Commands,
    time: Res<Time>,
) {
    local.timer.tick(time.delta());
    if local.timer.just_finished() {
        local.in_anim = false;
        print!("reset")
    }
    if !(local.in_anim) {
        if keys.pressed(KeyCode::ArrowRight) {
            let upd_pos = local.pos_vec + vec3(40., 0., 0.);
            let tween = Tween::new(
                // Use a quadratic easing on both endpoints.
                EaseFunction::QuadraticInOut,
                // Animation time (one way only; for ping-pong it takes 2 seconds
                // to come back to start).
                Duration::from_millis(250),
                // The lens gives the Animator access to the Transform component,
                // to animate it. It also contains the start and end values associated
                // with the animation ratios 0. and 1.
                TransformPositionLens {
                    start: local.pos_vec,
                    end: upd_pos,
                },
            );
            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.pos_vec = upd_pos;
            local.timer.reset();
            local.in_anim = true
        } else if keys.pressed(KeyCode::ArrowLeft) {
            let upd_pos = local.pos_vec + vec3(-40., 0., 0.);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(250),
                TransformPositionLens {
                    start: local.pos_vec,
                    end: upd_pos,
                },
            );
            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.pos_vec = upd_pos;
            local.timer.reset();
            local.in_anim = true
        } else if keys.pressed(KeyCode::ArrowDown) {
            let upd_pos = local.pos_vec + vec3(0., -40., 0.);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(250),
                TransformPositionLens {
                    start: local.pos_vec,
                    end: upd_pos,
                },
            );
            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.pos_vec = upd_pos;
            local.timer.reset();
            local.in_anim = true
        } else if keys.pressed(KeyCode::ArrowUp) {
            let upd_pos = local.pos_vec + vec3(0., 40., 0.);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(250),
                TransformPositionLens {
                    start: local.pos_vec,
                    end: upd_pos,
                },
            );

            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.pos_vec = upd_pos;
            local.timer.reset();
            local.in_anim = true
        }
    }
}

fn spawn_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3::ZERO,
            end: Vec3::new(0., 200., 0.),
        },
    )
    .with_repeat_count(RepeatCount::Finite(2))
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: bevy::color::Color::WHITE,
                    custom_size: Some(Vec2::new(40., 40.)),
                    ..default()
                },

                ..default()
            },
            Animator::new(tween),
        ))
        .id();
    commands.insert_resource(PosVar {
        in_anim: false,
        pos_vec: Vec3::ZERO,
        id: id,
        timer: Timer::from_seconds(0.25, TimerMode::Once),
    });
}
