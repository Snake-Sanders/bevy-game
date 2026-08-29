use bevy::{
    input::keyboard::{self, Key},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(
        asset_server.load("sprites/ball_blue_large.png"),
    ));
}

pub fn move_player(keyboard: Res<ButtonInput<KeyCode>>, key_input: Res<ButtonInput<Key>>) {
    if keyboard.pressed(KeyCode::KeyW) {
        info!("'w' is pressed")
    }
}
