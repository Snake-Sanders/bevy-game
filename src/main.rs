use bevy::prelude::*;

// The player sprite size
pub const PLAYER_SIZE: f32 = 64.0;

// Distance from the players center to the borders
pub const PLAYER_PADDING: f32 = PLAYER_SIZE / 2.0;

// The player speed
pub const PLAYER_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, confine_player))
        .run();
}

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_img = asset_server.load("sprites/ball_blue_large.png");
    commands.spawn(Camera2d);
    commands.spawn((Sprite::from_image(ball_img), Player));
}

/// moves the player on keypress A, S, D, W
pub fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }

    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if direction.length() > 0.0 {
        player.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

// constrains the player to go out of the window's boundaires
pub fn confine_player(mut player: Single<&mut Transform, With<Player>>, window: Single<&Window>) {
    let window_top = window.height() / 2.0;
    let window_bottom = -window_top;
    let window_right = window.width() / 2.0;
    let window_left = -window_right;

    let mut translation = player.translation;

    info!("{} {} w: {}", translation.x, translation.y, window.height());

    if translation.y + PLAYER_PADDING > window_top {
        translation.y = window_top - PLAYER_PADDING;
        warn!("touched ceiling");
    } else if translation.y - PLAYER_PADDING < window_bottom {
        translation.y = window_bottom + PLAYER_PADDING;
        warn!("touched ground");
    }

    if translation.x + PLAYER_PADDING > window_right {
        warn!("touched right");
        translation.x = window_right - PLAYER_PADDING;
    } else if translation.x - PLAYER_PADDING < window_left {
        warn!("touched left");
        translation.x = window_left + PLAYER_PADDING;
    }

    player.translation = translation;
}
