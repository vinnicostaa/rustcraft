use bevy::prelude::*;
use rc_render::RenderStartupSet;

use crate::{WorldConfig, spawn::spawn_initial_chunk};

/// World/terrain plugin.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConfig>().add_systems(
            Startup,
            spawn_initial_chunk.after(RenderStartupSet::PrepareAssets),
        );
    }
}
