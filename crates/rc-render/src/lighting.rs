use bevy::prelude::*;

use crate::RenderConfig;

pub(crate) fn setup_lighting(mut commands: Commands, config: Res<RenderConfig>) {
    commands.spawn((
        DirectionalLight {
            illuminance: config.sun_illuminance,
            shadows_enabled: config.shadows_enabled,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
