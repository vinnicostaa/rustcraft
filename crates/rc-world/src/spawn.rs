use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use rc_render::{BlockRenderAssets, RenderConfig, build_chunk_mesh_data};
use rc_voxel::ChunkCoord;

use crate::{
    ChunkMap, GeneratedChunk, WorldConfig,
    diagnostics::{CHUNK_FACES, CHUNK_VERTICES, CHUNKS_RENDERED},
    generation::{TerrainGenerator, generate_chunk},
};

/// Spawna o chunk inicial como uma única entidade renderizável.
///
/// A geometria do chunk fica em `rc-render`; `rc-world` apenas gera os dados de
/// mundo, registra a mesh no asset storage do Bevy e cria a entidade.
pub(crate) fn spawn_initial_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut diagnostics: Diagnostics,
    mut chunk_map: ResMut<ChunkMap>,
    block_assets: Res<BlockRenderAssets>,
    render_config: Res<RenderConfig>,
    world_config: Res<WorldConfig>,
) {
    let chunk_size = world_config.chunk_size;
    let max_height = world_config.max_height;
    let block_size = render_config.block_size;

    let generator = TerrainGenerator::new(world_config.seed, max_height);
    let chunk_coord = ChunkCoord::new(0, 0, 0);
    let chunk = generate_chunk(chunk_coord, chunk_size, &generator);
    let origin = chunk_coord.origin_block_pos(chunk_size);

    let chunk_mesh_data = build_chunk_mesh_data(&chunk, block_size);

    let face_count = chunk_mesh_data.face_count();
    let vertex_count = chunk_mesh_data.positions.len();

    diagnostics.add_measurement(&CHUNKS_RENDERED, || 1.0);
    diagnostics.add_measurement(&CHUNK_FACES, || face_count as f64);
    diagnostics.add_measurement(&CHUNK_VERTICES, || vertex_count as f64);

    let chunk_mesh = chunk_mesh_data.into_mesh();
    let chunk_mesh = meshes.add(chunk_mesh);

    let entity = commands
        .spawn((
            GeneratedChunk { coord: chunk_coord },
            Mesh3d(chunk_mesh),
            MeshMaterial3d(block_assets.chunk_material()),
            Transform::from_xyz(
                origin.x as f32 * block_size,
                origin.y as f32 * block_size,
                origin.z as f32 * block_size,
            ),
        ))
        .id();
    chunk_map.insert(chunk_coord, chunk, entity);
}
