use bevy::prelude::*;
use rc_inventory::SelectedBlock;
use rc_voxel::BlockState;
use rc_world::{ChunkMap, WorldConfig};

use crate::aimed_block::AimedBlock;

pub(crate) fn apply_block_interaction(
    aimed_block: Res<AimedBlock>,
    selected_block: Res<SelectedBlock>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut chunk_map: ResMut<ChunkMap>,
    world_config: Res<WorldConfig>,
) {
    let Some(hit) = aimed_block.hit() else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) {
        chunk_map.set_block(hit.target_pos, BlockState::air(), world_config.chunk_size);
    } else if mouse.just_pressed(MouseButton::Right) {
        chunk_map.set_block(
            hit.adjacent_pos,
            selected_block.block(),
            world_config.chunk_size,
        );
    }
}
