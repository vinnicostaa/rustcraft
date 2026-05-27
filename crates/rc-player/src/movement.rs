use bevy::prelude::*;
use rc_input::{ActionState, PlayerAction};

use crate::{Player, PlayerConfig};

pub(crate) fn move_player(
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
