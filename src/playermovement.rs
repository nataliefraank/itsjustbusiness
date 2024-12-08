use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "It's Just Business".to_string(),
                resolution: (640.0, 400.0).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_floor)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rigid_body = RigidBody::Dynamic;
    let collider = Collider::cuboid(0.5, 0.5);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("janitor-v1.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(rigid_body)
        .insert(collider)
        .insert(Velocity::zero())
        .insert(Player { speed: 200.0 })
        .insert(RigidBody::Dynamic)
        .insert(Player { speed: 3.5 });

    // This uses a camera which we haven't spawned in this mod.
    // .with_children(|parent| {
    //     parent.spawn(new_camera_2d());
    // });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>, // Use Input for keyboard input
    time: Res<Time>,                     // Use Time for frame time
    mut query: Query<(&Player, &mut Velocity)>, // Query for player components
) {
    for (player, mut velocity) in query.iter_mut() {
        let mut direction: Vec2 = Vec2::ZERO;

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

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        velocity.linvel = direction * player.speed * time.delta_seconds();
    }
}

fn spawn_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let width = 10.;
    let height = 1.;

    let rigid_body = RigidBody::Dynamic;
    let collider = Collider::cuboid(0.5, 0.5);

    commands
        .spawn(SpriteBundle {
            //material: materials.add(Color(0.7, 0.7, 0.7).into()),
            sprite: Sprite::new(Vec2::new(width, height)),
            ..Default::default()
        })
        .insert(rigid_body)
        .insert(collider)
        .insert(RigidBodyPositionSync::Discrete);
}

// // use bevy::a11y::accesskit::Point;
// use bevy::input::common_conditions::{self, *};
// use bevy::prelude::*;

// #[derive(Component)]
// // struct Player {
// //     position: Point,
// //     sprite: Rect,
// //     speed: i32,
// // }

// fn main() {
//     // let mut app = App::new();
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_system(player_movement.system())
//         .insert(Player { speed: 3.5 })
//         .run();
// }

// // System for handling player movement
// fn player_movement(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     let mut player_transform = query.single_mut();
//     let mut direction = Vec3::ZERO;

//     if keyboard_input.pressed(KeyCode::KeyW) {
//         direction.y += 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyS) {
//         direction.y -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyA) {
//         direction.x -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) {
//         direction.x += 1.0;
//     }

//     // Normalize direction to ensure consistent movement and scale with speed
//     if direction.length_squared() > 0.0 {
//         direction = direction.normalize();
//     }

//     let speed = 5.0;
//     let delta_time = 1.0 / 60.0; // Assuming a fixed frame rate for simplicity
//     player_transform.translation += direction * speed * delta_time;
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn(SpriteBundle {
//             texture: asset_server.load("janitor-v1.png"),
//             transform: Transform::from_xyz(30.0, 3.0, 0.0),
//             ..default()
//         })
//         .with_children(|parent| {
//             parent.spawn(new_camera_2d());
//         })
//         // .insert(Player {
//         //     position: Point { x: 0.0, y: 0.0 },
//         //     sprite: Rect {
//         //         min: ,
//         //         max: todo!(),
//         //     },car
//         //     speed: 3,
//         // })
//         .insert(TransformBundle::from(Transform::default()))
//         .insert(VisibilityBundle::default());

// let rigid_body = RigidBodyBundle {
//     mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
//     activation: RigidBodyActivation::cannot_sleep(),
//     ccd: RigidBodyCcd {
//         ccd_enabled: true,
//         ..Default::default()
//     },
//     ..Default::default()
// };
// let collider = ColliderBundle {
//     shape: ColliderShape::cuboid(0.5, 0.5),
//     flags: ColliderFlags {
//         active_events: ActiveEvents::CONTACT_FORCE_EVENTS,
//         ..Default::default()
//     },
//     ..Default::default()
// };
