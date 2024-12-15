use bevy::{prelude::*, render::camera::ScalingMode, transform::{self, commands}, window::PrimaryWindow};
use bevy_ecs_tiled::{TiledMapHandle, TiledMapPlugin};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEvent;
use bevy_tweening::Tween;
// use r#move::{derive_z_from_y_after_move, move_camera, move_player};
use std::time::Duration;

fn check_for_collisions(
    mut commands: Commands,
    mut sprite: SpriteBundle,
    mut tasks: Vec<SpriteBundle>,
    player_query: Query<(Entity, &Transform), With<Task>,
    collider_query: Query<&Transform, With<SpriteBundle>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (sprite.transform) = tas_query.into_inner();
}