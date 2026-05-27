use bevy::prelude::*;
use rc_voxel::{BlockState, DIRT, GRASS, STONE};

use crate::{RenderConfig, materials::block_material};

/// Shared handles for block rendering.
#[derive(Resource, Clone)]
pub struct BlockRenderAssets {
    block_mesh: Handle<Mesh>,
    grass_material: Handle<StandardMaterial>,
    dirt_material: Handle<StandardMaterial>,
    stone_material: Handle<StandardMaterial>,
}

impl BlockRenderAssets {
    pub fn block_mesh(&self) -> Handle<Mesh> {
        self.block_mesh.clone()
    }

    pub fn material_for(&self, block: BlockState) -> Option<Handle<StandardMaterial>> {
        match block.id {
            GRASS => Some(self.grass_material.clone()),
            DIRT => Some(self.dirt_material.clone()),
            STONE => Some(self.stone_material.clone()),
            _ => None,
        }
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
