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

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, (spawn_entity)) // Add a startup system to spawn the entity
        .add_systems(Update, keyboard_input)
        .run();
}

#[derive(Resource)]
struct posVar {
    posVec: Vec3,
    id: Entity,
    timer: Timer,
    inAnim: bool,
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform>,
    mut local: ResMut<posVar>,
    mut commands: Commands,
    time: Res<Time>,
) {
    local.timer.tick(time.delta());
    if local.timer.just_finished() {
        local.inAnim = false;
        print!("reset")
    }
    if !(local.inAnim) {
        if keys.pressed(KeyCode::ArrowRight) {
            let updPos = local.posVec + vec3(40., 0., 0.);
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
                    start: local.posVec,
                    end: updPos,
                },
            );
            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.posVec = updPos;
            local.timer.reset();
            local.inAnim = true
        } else if keys.pressed(KeyCode::ArrowLeft) {
            let updPos = local.posVec + vec3(-40., 0., 0.);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(250),
                TransformPositionLens {
                    start: local.posVec,
                    end: updPos,
                },
            );
            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.posVec = updPos;
            local.timer.reset();
            local.inAnim = true
        } else if keys.pressed(KeyCode::ArrowDown) {
            let updPos = local.posVec + vec3(0., -40., 0.);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(250),
                TransformPositionLens {
                    start: local.posVec,
                    end: updPos,
                },
            );
            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.posVec = updPos;
            local.timer.reset();
            local.inAnim = true
        } else if keys.pressed(KeyCode::ArrowUp) {
            let updPos = local.posVec + vec3(0., 40., 0.);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(250),
                TransformPositionLens {
                    start: local.posVec,
                    end: updPos,
                },
            );

            println!("moving");

            commands
                .entity(local.id)
                .remove::<Animator<Transform>>()
                .insert(Animator::new(tween));
            local.posVec = updPos;
            local.timer.reset();
            local.inAnim = true
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
    commands.insert_resource(posVar {
        inAnim: false,
        posVec: Vec3::ZERO,
        id: id,
        timer: Timer::from_seconds(0.25, TimerMode::Once),
    });
}
