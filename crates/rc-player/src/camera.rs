use bevy::prelude::*;

use crate::{Player, PlayerConfig};

pub(crate) fn spawn_player(mut commands: Commands, config: Res<PlayerConfig>) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_translation(config.spawn_position).looking_at(config.look_at, Vec3::Y),
    ));
}
