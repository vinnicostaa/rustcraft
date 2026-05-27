use bevy::prelude::*;
use rc_render::{BlockRenderAssets, RenderConfig, RenderStartupSet};
use rc_voxel::BlockType;

/// Componente lógico para uma entidade de bloco gerada.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub kind: BlockType,
}

/// Marca os blocos gerados para o protótipo inicial.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedChunkBlock;

/// Configurações iniciais de geração do mundo.
#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldConfig {
    pub chunk_size: i32,
    pub max_height: i32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            chunk_size: 24,
            max_height: 6,
        }
    }
}

/// World/terrain plugin.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConfig>().add_systems(
            Startup,
            spawn_initial_chunk.after(RenderStartupSet::PrepareAssets),
        );
    }
}

fn spawn_initial_chunk(
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
                let kind = BlockType::for_layer(y, surface_y);

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

/// Função protótipo simples para altura do terreno.
///
/// Retorna uma altura entre `0` e `altura_máxima`.
pub fn terrain_height(x: i32, z: i32, max_height: i32) -> i32 {
    if max_height <= 0 {
        return 0;
    }

    let fx = x as f32 * 0.3;
    let fz = z as f32 * 0.3;

    let noise = (fx.sin() + fz.cos() + (fx * 0.5).cos()) / 3.0;
    let normalized = (noise + 1.0) / 2.0;

    (normalized * max_height as f32) as i32
}

#[cfg(test)]
mod tests {
    use super::terrain_height;

    #[test]
    fn terrain_height_stays_within_expected_range() {
        let max_height = 6;

        for x in -16..16 {
            for z in -16..16 {
                let height = terrain_height(x, z, max_height);
                assert!(
                    (0..=max_height).contains(&height),
                    "height {height} was outside 0..={max_height} at {x},{z}"
                );
            }
        }
    }

    #[test]
    fn terrain_height_handles_non_positive_max_height() {
        assert_eq!(terrain_height(0, 0, 0), 0);
        assert_eq!(terrain_height(0, 0, -10), 0);
    }
}
