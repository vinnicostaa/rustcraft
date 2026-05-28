use bevy::prelude::*;
use rc_render::{BlockRenderAssets, RenderConfig};
use rc_voxel::ChunkCoord;

use crate::{
    Block, GeneratedChunkBlock, WorldConfig,
    generation::{TerrainGenerator, generate_chunk},
};

pub(crate) fn spawn_initial_chunk(
    mut commands: Commands,
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

    for x in 0..chunk.size() {
        for y in 0..chunk.size() {
            for z in 0..chunk.size() {
                let Some(kind) = chunk.get(x, y, z) else {
                    continue;
                };

                if kind.is_air() {
                    continue;
                }

                if let Some(material) = block_assets.material_for(kind) {
                    commands.spawn((
                        Block { kind },
                        GeneratedChunkBlock,
                        Mesh3d(block_assets.block_mesh()),
                        MeshMaterial3d(material),
                        Transform::from_xyz(
                            (origin.x + x) as f32 * block_size,
                            (origin.y + y) as f32 * block_size,
                            (origin.z + z) as f32 * block_size,
                        ),
                    ));
                }
            }
        }
    }
}
