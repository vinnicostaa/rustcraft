use bevy::prelude::*;
use rc_render::{BlockRenderAssets, RenderConfig};
use rc_voxel::block_for_layer;

use crate::{Block, GeneratedChunkBlock, WorldConfig, generation::terrain_height};

pub(crate) fn spawn_initial_chunk(
    mut commands: Commands,
    block_assets: Res<BlockRenderAssets>,
    render_config: Res<RenderConfig>,
    world_config: Res<WorldConfig>,
) {
    let chunk_size = world_config.chunk_size;
    let max_height = world_config.max_height;
    let block_size = render_config.block_size;

    for x in 0..chunk_size {
        for z in 0..chunk_size {
            let surface_y = terrain_height(x, z, max_height);

            for y in 0..=surface_y {
                let kind = block_for_layer(y, surface_y);

                if let Some(material) = block_assets.material_for(kind) {
                    commands.spawn((
                        Block { kind },
                        GeneratedChunkBlock,
                        Mesh3d(block_assets.block_mesh()),
                        MeshMaterial3d(material),
                        Transform::from_xyz(
                            x as f32 * block_size,
                            y as f32 * block_size,
                            z as f32 * block_size,
                        ),
                    ));
                }
            }
        }
    }
}
