use bevy::prelude::Component;
use rc_voxel::BlockState;

/// Componente lógico para uma entidade de bloco gerada.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub kind: BlockState,
}

/// Marca os blocos gerados para o protótipo inicial.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedChunkBlock;
