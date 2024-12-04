// use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
// pub use camera::*;

// struct Player {
//     speed: f32,
// }

// fn main() {
//     App::new()
//         .insert_resource(WindowDescriptor {
//             title: "Platformer!".to_string(),
//             width: 640.0,
//             height: 400.0,
//             vsync: true,
//             ..Default::default()
//         })
//         .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
//         .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
//         .add_startup_stage("player_setup", SystemStage::single(spawn_player.system()))
//         .add_startup_stage("floor_setup", SystemStage::single(spawn_floor.system()))
//         .add_system(player_jumps.system())
//         .add_system(player_movement.system())
//         .add_system(jump_reset.system())
//         .add_plugins(DefaultPlugins)
//         .run();
// }

// fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
//     let rigid_body = RigidBodyBundle {
//         mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
//         activation: RigidBodyActivation::cannot_sleep(),
//         ccd: RigidBodyCcd {
//             ccd_enabled: true,
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//     let collider = ColliderBundle {
//         shape: ColliderShape::cuboid(0.5, 0.5),
//         flags: ColliderFlags {
//             active_events: ActiveEvents::CONTACT_EVENTS,
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//     commands
//         .spawn_bundle(SpriteBundle {
//             material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
//             sprite: Sprite::new(Vec2::new(1.0, 1.0)),
//             ..Default::default()
//         })
//         .insert_bundle(rigid_body)
//         .insert_bundle(collider)
//         .insert(RigidBodyPositionSync::Discrete)
//         .insert(Player { speed: 3.5 })
//         .insert(Jumper {
//             jump_impulse: 7.,
//             is_jumping: false,
//         })
//         .with_children(|parent| {
//             parent.spawn_bundle(new_camera_2d());
//         });
// }

// fn player_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut players: Query<(&Player, &mut RigidBodyVelocity)>,
// ) {
//     for (player, mut velocity) in players.iter_mut() {
//         if keyboard_input.pressed(KeyCode::KeyW) {
//             velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
//         }
//         if keyboard_input.pressed(KeyCode::KeyA) {
//             velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
//         }
//     }
// }

// fn spawn_floor(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
//     let width = 10.;
//     let height = 1.;
//     let rigid_body = RigidBodyBundle {
//         position: Vec2::new(0.0, -2.).into(),
//         body_type: RigidBodyType::Static,
//         ..Default::default()
//     };
//     let collider = ColliderBundle {
//         shape: ColliderShape::cuboid(width / 2., height / 2.),
//         ..Default::default()
//     };
//     commands
//         .spawn(SpriteBundle {
//             material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
//             sprite: Sprite::new(Vec2::new(width, height)),
//             ..Default::default()
//         })
//         .insert_bundle(rigid_body)
//         .insert_bundle(collider)
//         .insert(RigidBodyPositionSync::Discrete);
// }

// fn jump_reset(
//     mut query: Query<(Entity, &mut Jumper)>,
//     mut contact_events: EventReader<ContactEvent>,
// ) {
//     for contact_event in contact_events.iter() {
//         for (entity, mut jumper) in query.iter_mut() {
//             if let ContactEvent::Started(h1, h2) = contact_event {
//                 if h1.entity() == entity || h2.entity() == entity {
//                     jumper.is_jumping = false
//                 }
//             }
//         }
//     }
// }

// // // use bevy::a11y::accesskit::Point;
// // use bevy::input::common_conditions::{self, *};
// // use bevy::prelude::*;

// // #[derive(Component)]
// // // struct Player {
// // //     position: Point,
// // //     sprite: Rect,
// // //     speed: i32,
// // // }

// // struct Player {
// //     speed: f32,
// // }

// // fn main() {
// //     // let mut app = App::new();
// //     App::new()
// //         .add_plugins(DefaultPlugins)
// //         .add_systems(Startup, setup)
// //         .add_system(player_movement.system())
// //         .insert(Player { speed: 3.5 })
// //         .run();
// // }

// // // System for handling player movement
// // fn player_movement(
// //     keyboard_input: Res<ButtonInput<KeyCode>>,
// //     mut query: Query<&mut Transform, With<Player>>,
// // ) {
// //     let mut player_transform = query.single_mut();
// //     let mut direction = Vec3::ZERO;

// //     if keyboard_input.pressed(KeyCode::KeyW) {
// //         direction.y += 1.0;
// //     }
// //     if keyboard_input.pressed(KeyCode::KeyS) {
// //         direction.y -= 1.0;
// //     }
// //     if keyboard_input.pressed(KeyCode::KeyA) {
// //         direction.x -= 1.0;
// //     }
// //     if keyboard_input.pressed(KeyCode::KeyD) {
// //         direction.x += 1.0;
// //     }

// //     // Normalize direction to ensure consistent movement and scale with speed
// //     if direction.length_squared() > 0.0 {
// //         direction = direction.normalize();
// //     }

// //     let speed = 5.0;
// //     let delta_time = 1.0 / 60.0; // Assuming a fixed frame rate for simplicity
// //     player_transform.translation += direction * speed * delta_time;
// // }

// // fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
// //     commands
// //         .spawn(SpriteBundle {
// //             texture: asset_server.load("janitor-v1.png"),
// //             transform: Transform::from_xyz(30.0, 3.0, 0.0),
// //             ..default()
// //         })
// //         .with_children(|parent| {
// //             parent.spawn(new_camera_2d());
// //         })
// //         // .insert(Player {
// //         //     position: Point { x: 0.0, y: 0.0 },
// //         //     sprite: Rect {
// //         //         min: ,
// //         //         max: todo!(),
// //         //     },car
// //         //     speed: 3,
// //         // })
// //         .insert(TransformBundle::from(Transform::default()))
// //         .insert(VisibilityBundle::default());

// //     commands.spawn((
// //         // Player {
// //         //     position: Point { x: 0.0, y: 0.0 },
// //         //     sprite: Rect {},
// //         //     speed: 3,
// //         // },
// //         Transform::default(),
// //         GlobalTransform::default(),
// //         Visibility::default(),
// //         InheritedVisibility::default(),
// //     ));
// // }
