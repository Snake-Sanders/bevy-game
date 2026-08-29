use bevy::prelude::*;
use rand::random;

// The player sprite size
pub const PLAYER_SIZE: f32 = 64.0;

// Distance from the players center to the borders
pub const PLAYER_PADDING: f32 = PLAYER_SIZE / 2.0;

// The player speed
pub const PLAYER_SPEED: f32 = 500.0;

pub const NUMBER_OF_ENEMIE: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_enemies))
        .add_systems(Update, (move_player, confine_player))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

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

    if translation.y + PLAYER_PADDING > window_top {
        translation.y = window_top - PLAYER_PADDING;
    } else if translation.y - PLAYER_PADDING < window_bottom {
        translation.y = window_bottom + PLAYER_PADDING;
    }

    if translation.x + PLAYER_PADDING > window_right {
        translation.x = window_right - PLAYER_PADDING;
    } else if translation.x - PLAYER_PADDING < window_left {
        translation.x = window_left + PLAYER_PADDING;
    }

    player.translation = translation;
}

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let enemy_img = asset_server.load("sprites/ball_red_large.png");
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    (0..NUMBER_OF_ENEMIE).for_each(|i| {
        // 0.0 is at the center of the window, consider the negative areas.
        let x: f32 = random::<f32>() * half_width * gen_sign();
        let y: f32 = random::<f32>() * half_height * gen_sign();

        commands.spawn((
            Sprite::from_image(enemy_img.clone()),
            Transform::from_xyz(x, y, 0.0),
            Enemy,
        ));

        info!("Spawning enemy {i}");
    });
}

fn gen_sign() -> f32 {
    if rand::random::<bool>() { 1.0 } else { -1.0 }
}
