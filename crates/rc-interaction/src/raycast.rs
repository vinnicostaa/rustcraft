use bevy::{
    ecs::system::SystemParam,
    picking::prelude::{MeshRayCast, MeshRayCastSettings, RayCastVisibility},
    prelude::*,
};
use rc_player::Player;
use rc_render::RenderConfig;
use rc_voxel::{adjacent_block_pos_from_hit, block_pos_from_hit};
use rc_world::GeneratedChunk;

use crate::aimed_block::{AimedBlock, AimedBlockHit};

const PLAYER_REACH: f32 = 8.0;

/// Parâmetros agrupados para descobrir o bloco mirado pelo player.
///
/// `#[derive(SystemParam)]` permite agrupar múltiplos parâmetros Bevy em uma
/// struct, reduzindo a contagem de argumentos da função de sistema e nomeando
/// o conjunto de forma legível.
#[derive(SystemParam)]
pub(crate) struct AimedBlockRaycastParams<'w, 's> {
    player_query: Query<'w, 's, &'static GlobalTransform, With<Player>>,
    chunk_query: Query<'w, 's, (), With<GeneratedChunk>>,
    render_config: Res<'w, RenderConfig>,
}

pub(crate) fn update_aimed_block(
    params: AimedBlockRaycastParams,
    mut ray_cast: MeshRayCast,
    mut aimed_block: ResMut<AimedBlock>,
) {
    aimed_block.clear();
    let player_transform = {
        let Ok(player_transform) = params.player_query.single() else {
            return;
        };

        player_transform.compute_transform()
    };

    let ray = Ray3d::new(player_transform.translation, player_transform.forward());

    let filter = |entity| params.chunk_query.contains(entity);
    let settings = MeshRayCastSettings::default()
        .with_filter(&filter)
        .with_visibility(RayCastVisibility::Visible);

    let Some((_, hit)) = ray_cast.cast_ray(ray, &settings).first() else {
        return;
    };

    if hit.distance > PLAYER_REACH {
        return;
    }

    let normal = hit.normal.normalize_or_zero();

    let Some(target_pos) = block_pos_from_hit(
        hit.point.to_array(),
        normal.to_array(),
        params.render_config.block_size,
    ) else {
        return;
    };

    let Some(adjacent_pos) = adjacent_block_pos_from_hit(
        hit.point.to_array(),
        normal.to_array(),
        params.render_config.block_size,
    ) else {
        return;
    };

    // Guarda apenas o alvo atual; a ação de quebrar/colocar fica em outro sistema.
    aimed_block.set(AimedBlockHit {
        target_pos,
        adjacent_pos,
    });
}
