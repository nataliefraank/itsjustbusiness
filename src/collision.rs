use bevy::{prelude::*, render::camera::ScalingMode, transform::{self, commands}, window::PrimaryWindow};
use bevy_ecs_tiled::{TiledMapHandle, TiledMapPlugin};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEvent;
use bevy_tweening::Tween;
// use r#move::{derive_z_from_y_after_move, move_camera, move_player};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn check_for_collisions(
    mut commands: Commands,
    mut sprite: SpriteBundle,
    mut tasks: Vec<SpriteBundle>,
    mut player_query: Query<(Entity, &Transform), With<Task>,
    mut collider_query: Query<&Transform, With<SpriteBundle>
    
) {
    let (sprite) = player_query.into_inner();

    for (collider_entity, collider_transform, in &collider_query) 
    {
        let collision = player_collision(sprite.transform);
        if let Some(collision) = collision {
            collision_events.send_default();
        
    };
}

fn player_collision(player: Transform,) -> Option<Collision> {
    if !player.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(player.center());
    let offset = player.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}


