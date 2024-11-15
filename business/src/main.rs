use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::prelude::*;

#[derive(Component)]
struct Person;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, hello_world).run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("hello world!");
}

