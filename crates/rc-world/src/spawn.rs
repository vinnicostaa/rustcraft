use bevy::prelude::*;
use rc_render::{BlockRenderAssets, RenderConfig, build_chunk_mesh};
use rc_voxel::ChunkCoord;

use crate::{
    GeneratedChunk, WorldConfig,
    generation::{TerrainGenerator, generate_chunk},
};

/// Spawna o chunk inicial como uma única entidade renderizável.
///
/// A geometria do chunk fica em `rc-render`; `rc-world` apenas gera os dados de
/// mundo, registra a mesh no asset storage do Bevy e cria a entidade.
pub(crate) fn spawn_initial_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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

    let chunk_mesh = build_chunk_mesh(&chunk, block_size);
    let chunk_mesh = meshes.add(chunk_mesh);

    commands.spawn((
        GeneratedChunk { coord: chunk_coord },
        Mesh3d(chunk_mesh),
        MeshMaterial3d(block_assets.chunk_material()),
        Transform::from_xyz(
            origin.x as f32 * block_size,
            origin.y as f32 * block_size,
            origin.z as f32 * block_size,
        ),
    ));
}
