use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_img = asset_server.load("sprites/ball_blue_large.png");
    commands.spawn(Camera2d);
    commands.spawn((Sprite::from_image(ball_img), Player));
}

pub const PLAYER_SPEED: f32 = 500.0;

pub fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        info!("pressed W");
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        info!("pressed S");
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        info!("pressed A");
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }

    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        info!("pressed D");
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if direction.length() > 0.0 {
        player.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}
