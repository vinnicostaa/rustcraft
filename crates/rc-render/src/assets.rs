use bevy::prelude::*;
use rc_voxel::{BlockState, DIRT, GRASS, STONE};

use crate::{RenderConfig, materials::block_material};

/// Handles compartilhados para renderização voxel.
///
/// O recurso centraliza meshes e materiais criados no startup para que outros
/// sistemas usem handles baratos em vez de recriar assets.
#[derive(Resource, Clone)]
pub struct BlockRenderAssets {
    block_mesh: Handle<Mesh>,
    grass_material: Handle<StandardMaterial>,
    dirt_material: Handle<StandardMaterial>,
    stone_material: Handle<StandardMaterial>,
}

impl BlockRenderAssets {
    /// Mesh cúbica usada pelo caminho legado de renderização por bloco.
    pub fn block_mesh(&self) -> Handle<Mesh> {
        self.block_mesh.clone()
    }

    /// Material associado a um estado de bloco conhecido.
    pub fn material_for(&self, block: BlockState) -> Option<Handle<StandardMaterial>> {
        match block.id {
            GRASS => Some(self.grass_material.clone()),
            DIRT => Some(self.dirt_material.clone()),
            STONE => Some(self.stone_material.clone()),
            _ => None,
        }
    }

    /// Material temporário para a mesh agregada do chunk.
    ///
    /// Uma única mesh Bevy usa um material simples neste estágio. Atlas,
    /// vertex color ou outro caminho de material por face entram depois sem
    /// voltar ao spawn de uma entidade por bloco.
    pub fn chunk_material(&self) -> Handle<StandardMaterial> {
        self.grass_material.clone()
    }
}

pub(crate) fn setup_block_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<RenderConfig>,
) {
    let block_size = config.block_size;

    let block_mesh = meshes.add(Cuboid::new(block_size, block_size, block_size));
    let grass_material = materials.add(block_material(Color::srgb(0.2, 0.6, 0.1)));
    let dirt_material = materials.add(block_material(Color::srgb(0.4, 0.25, 0.1)));
    let stone_material = materials.add(block_material(Color::srgb(0.5, 0.5, 0.5)));

    commands.insert_resource(BlockRenderAssets {
        block_mesh,
        grass_material,
        dirt_material,
        stone_material,
    });
}
