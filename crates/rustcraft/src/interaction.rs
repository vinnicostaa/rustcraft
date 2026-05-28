use bevy::{
    picking::prelude::{MeshRayCast, MeshRayCastSettings, RayCastVisibility},
    prelude::*,
};
use rc_player::Player;
use rc_render::RenderConfig;
use rc_voxel::block_pos_from_hit;
use rc_world::GeneratedChunk;

const PLAYER_REACH: f32 = 8.0;

/// Sistemas de interação entre o player e o mundo.
pub struct RustcraftInteractionPlugin;

impl Plugin for RustcraftInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            draw_player_chunk_raycast.after(TransformSystems::Propagate),
        );
    }
}

fn draw_player_chunk_raycast(
    player_query: Query<&GlobalTransform, With<Player>>,
    chunk_query: Query<(), With<GeneratedChunk>>,
    render_config: Res<RenderConfig>,
    mut ray_cast: MeshRayCast,
    mut gizmos: Gizmos,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_transform = player_transform.compute_transform();
    let ray = Ray3d::new(player_transform.translation, player_transform.forward());

    let filter = |entity| chunk_query.contains(entity);
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

    let Some(block_pos) = block_pos_from_hit(
        hit.point.to_array(),
        normal.to_array(),
        render_config.block_size,
    ) else {
        return;
    };

    // Highlight visual temporário: por enquanto isso é ferramenta de debug, não
    // estado de gameplay persistente.
    let block_center = Vec3::new(
        (block_pos.x as f32 + 0.5) * render_config.block_size,
        (block_pos.y as f32 + 0.5) * render_config.block_size,
        (block_pos.z as f32 + 0.5) * render_config.block_size,
    );

    gizmos.cube(
        Transform::from_translation(block_center)
            .with_scale(Vec3::splat(render_config.block_size * 1.02)),
        Color::srgb(0.0, 1.0, 1.0),
    );
}
