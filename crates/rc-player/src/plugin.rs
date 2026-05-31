use bevy::prelude::*;

use crate::{PlayerConfig, camera::spawn_player, look::look_player, movement::move_player};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum PlayerControlState {
    /// Mouse look e movimento aceitam input.
    #[default]
    Enabled,
    /// Mouse look e movimento ficam pausados.
    Disabled,
}

/// Plugin do controlador de câmera/player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .init_state::<PlayerControlState>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (look_player, move_player)
                    .chain()
                    .run_if(in_state(PlayerControlState::Enabled)),
            );
    }
}
