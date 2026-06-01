use bevy::prelude::*;
use rc_render::RenderConfig;
use rc_voxel::BlockPos;

use crate::aimed_block::AimedBlock;

pub(crate) fn draw_aimed_block_gizmo(
    aimed_block: Res<AimedBlock>,
    render_config: Res<RenderConfig>,
    mut gizmos: Gizmos,
) {
    let Some(hit) = aimed_block.hit() else {
        return;
    };

    let block_center = block_center_from_pos(hit.target_pos, render_config.block_size);

    gizmos.cube(
        Transform::from_translation(block_center)
            .with_scale(Vec3::splat(render_config.block_size * 1.01)),
        Color::srgb(1.0, 0.9, 0.2),
    );
}

fn block_center_from_pos(pos: BlockPos, block_size: f32) -> Vec3 {
    Vec3::new(
        (pos.x as f32 + 0.5) * block_size,
        (pos.y as f32 + 0.5) * block_size,
        (pos.z as f32 + 0.5) * block_size,
    )
}
