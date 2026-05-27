use bevy::prelude::*;

use crate::{PlayerConfig, camera::spawn_player, movement::move_player};

/// Camera/player controller plugin.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}
