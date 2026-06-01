use bevy::prelude::*;

use crate::materials::block_material;

/// Handles compartilhados para renderização voxel.
///
/// O recurso centraliza meshes e materiais criados no startup para que outros
/// sistemas usem handles baratos em vez de recriar assets.
#[derive(Resource, Clone)]
pub struct ChunkRenderAssets {
    chunk_material: Handle<StandardMaterial>,
}

impl ChunkRenderAssets {
    /// Material base para a mesh agregada do chunk.
    ///
    /// A cor visual atual vem dos vertex colors emitidos no meshing, então o
    /// material fica neutro. Atlas, array texture ou outro caminho de textura
    /// por face entram depois sem voltar ao spawn de uma entidade por bloco.
    pub fn chunk_material(&self) -> Handle<StandardMaterial> {
        self.chunk_material.clone()
    }
}

pub(crate) fn setup_chunk_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk_material = materials.add(block_material(Color::WHITE));

    commands.insert_resource(ChunkRenderAssets { chunk_material });
}
