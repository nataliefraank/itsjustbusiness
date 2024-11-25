use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::prelude::*;

#[derive(Component)]
struct Person;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("assets/janitor32x48.png"),..default()
    });
}

pub Struct playercoords{
    x: f64
    y: f64
    z: f64
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>
){
    if keys.just_pressed(KeyCode: Space)
    {
        //Perform Task
    }

    if keys.pressed(KeyCode: KeyA)
    {
        playercoords.x = playercoords.x - 2;
        player = Transform::from_xyz(playercoords.x, playercoords.y, playercoords.z);
    }

    if keys.pressed(KeyCode: KeyD)
    {
        //Move character right
    }
    
    if keys.pressed(KeyCode: KeyS)
    {
        //Move character down
    }

    if keys.pressed(KeyCode: KeyW)
    {
        //Move Character Up
    }
}