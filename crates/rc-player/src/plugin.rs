use bevy::prelude::*;

use crate::{
    PlayerConfig, camera::spawn_player, cursor::toggle_cursor_grab, look::look_player,
    movement::move_player,
};

/// Plugin do controlador de camera/player.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (toggle_cursor_grab, look_player, move_player).chain(),
            );
    }
}
