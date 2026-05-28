use crate::{
    ChunkMap, WorldConfig, diagnostics::register_world_diagnostics, remesh::rebuild_dirty_chunks,
    spawn::spawn_initial_chunk,
};
use bevy::prelude::*;
use rc_render::RenderStartupSet;

/// World/terrain plugin.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        register_world_diagnostics(app);

        app.init_resource::<WorldConfig>()
            .init_resource::<ChunkMap>()
            .add_systems(
                Startup,
                spawn_initial_chunk.after(RenderStartupSet::PrepareAssets),
            )
            .add_systems(PostUpdate, rebuild_dirty_chunks);
    }
}
