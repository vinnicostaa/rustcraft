use crate::{selection::SelectedBlock, state::GameState};
use bevy::{
    ecs::system::SystemParam,
    picking::prelude::{MeshRayCast, MeshRayCastSettings, RayCastVisibility},
    prelude::*,
};
use rc_player::Player;
use rc_render::RenderConfig;
use rc_voxel::{BlockState, adjacent_block_pos_from_hit, block_pos_from_hit};
use rc_world::{ChunkMap, GeneratedChunk, WorldConfig};

const PLAYER_REACH: f32 = 8.0;

/// Parâmetros de sistema agrupados para o raycast de interação.
///
/// `#[derive(SystemParam)]` permite agrupar múltiplos parâmetros Bevy em uma
/// struct, reduzindo a contagem de argumentos da função de sistema e nomeando
/// o conjunto de forma legível.
#[derive(SystemParam)]
struct InteractionParams<'w, 's> {
    player_query: Query<'w, 's, &'static GlobalTransform, With<Player>>,
    chunk_query: Query<'w, 's, (), With<GeneratedChunk>>,
    render_config: Res<'w, RenderConfig>,
    world_config: Res<'w, WorldConfig>,
    selected_block: Res<'w, SelectedBlock>,
    mouse: Res<'w, ButtonInput<MouseButton>>,
    chunk_map: ResMut<'w, ChunkMap>,
}

/// Sistemas de interação entre o player e o mundo.
pub struct RustcraftInteractionPlugin;

impl Plugin for RustcraftInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            draw_player_chunk_raycast
                .after(TransformSystems::Propagate)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn draw_player_chunk_raycast(
    mut params: InteractionParams,
    mut ray_cast: MeshRayCast,
    mut gizmos: Gizmos,
) {
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

    let ray_end = ray.origin + ray.direction * PLAYER_REACH;
    gizmos.line(ray.origin, ray_end, Color::srgb(1.0, 0.8, 0.0));

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

    // --- Interação de quebrar bloco ---
    if params.mouse.just_pressed(MouseButton::Left) {
        params.chunk_map.set_block(
            target_pos,
            BlockState::air(),
            params.world_config.chunk_size,
        );
    } else if params.mouse.just_pressed(MouseButton::Right) {
        params.chunk_map.set_block(
            adjacent_pos,
            params.selected_block.block(),
            params.world_config.chunk_size,
        );
    }

    // Highlight visual temporário: por enquanto isso é ferramenta de debug, não
    // estado de gameplay persistente.
    let block_center = Vec3::new(
        (target_pos.x as f32 + 0.5) * params.render_config.block_size,
        (target_pos.y as f32 + 0.5) * params.render_config.block_size,
        (target_pos.z as f32 + 0.5) * params.render_config.block_size,
    );

    gizmos.cube(
        Transform::from_translation(block_center)
            .with_scale(Vec3::splat(params.render_config.block_size * 1.02)),
        Color::srgb(0.0, 1.0, 1.0),
    );
}
