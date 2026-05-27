use bevy::prelude::*;
use rc_input::{ActionState, PlayerAction};

/// Controlled player/camera entity for the initial prototype.
#[derive(Component, Debug)]
pub struct Player;

/// Initial fly-camera controller settings.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerConfig {
    pub fly_speed: f32,
    pub spawn_position: Vec3,
    pub look_at: Vec3,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            fly_speed: 10.0,
            spawn_position: Vec3::new(8.0, 5.0, 20.0),
            look_at: Vec3::new(8.0, 0.0, 8.0),
        }
    }
}

/// Camera/player controller plugin.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands, config: Res<PlayerConfig>) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_translation(config.spawn_position).looking_at(config.look_at, Vec3::Y),
    ));
}

fn move_player(
    actions: Res<ActionState>,
    time: Res<Time>,
    config: Res<PlayerConfig>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let distance = config.fly_speed * time.delta_secs();
        let forward = transform.forward();
        let right = transform.right();

        if actions.pressed(PlayerAction::MoveForward) {
            transform.translation += forward * distance;
        }
        if actions.pressed(PlayerAction::MoveBackward) {
            transform.translation -= forward * distance;
        }
        if actions.pressed(PlayerAction::MoveLeft) {
            transform.translation -= right * distance;
        }
        if actions.pressed(PlayerAction::MoveRight) {
            transform.translation += right * distance;
        }
        if actions.pressed(PlayerAction::MoveUp) {
            transform.translation += Vec3::Y * distance;
        }
        if actions.pressed(PlayerAction::MoveDown) {
            transform.translation -= Vec3::Y * distance;
        }
    }
}
