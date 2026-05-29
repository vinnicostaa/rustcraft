use bevy::prelude::*;
use rc_render::{RenderConfig, build_chunk_mesh};

use crate::ChunkMap;

/// Reconstrói a mesh de cada chunk marcado como dirty no frame.
///
/// Coleta os coords dirty primeiro para evitar conflito de borrow mutável +
/// imutável no mesmo iterador. A flag dirty é limpa depois da mesh ser atualizada.
pub(crate) fn rebuild_dirty_chunks(
    mut chunk_map: ResMut<ChunkMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_query: Query<&Mesh3d>,
    render_config: Res<RenderConfig>,
) {
    let dirty: Vec<_> = chunk_map.dirty_coords().collect();

    for coord in dirty {
        let Some(entry) = chunk_map.get_mut(coord) else {
            continue;
        };

        let new_mesh = build_chunk_mesh(&entry.data, render_config.block_size);
        let entity = entry.entity;

        let Ok(mesh3d) = mesh_query.get(entity) else {
            continue;
        };

        let Some(mesh_asset) = meshes.get_mut(&mesh3d.0) else {
            continue;
        };

        *mesh_asset = new_mesh;
        entry.dirty = false;
    }
}
