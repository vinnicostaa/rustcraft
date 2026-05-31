use crate::{Player, PlayerConfig};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;

/// Gira o player/câmera a partir do movimento acumulado do mouse no frame.
///
/// A rotação usa a ordem YXZ: yaw no eixo Y, pitch no eixo X e roll preservado.
/// O pitch fica limitado para evitar que a câmera vire além da vertical.
pub(crate) fn look_player(
    mouse_motion: Res<AccumulatedMouseMotion>,
    config: Res<PlayerConfig>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let delta = mouse_motion.delta;

    if delta == Vec2::ZERO {
        return;
    }

    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw -= delta.x * config.mouse_sensitivity;
    pitch -= delta.y * config.mouse_sensitivity;
    pitch = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}
