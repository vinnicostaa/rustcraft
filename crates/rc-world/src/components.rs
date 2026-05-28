use bevy::prelude::Component;
use rc_voxel::{BlockState, ChunkCoord};

/// Componente lógico para o caminho legado de entidade por bloco.
///
/// Blocos comuns de terreno devem viver em `Chunk`; este componente permanece
/// disponível enquanto interações futuras ainda não têm um modelo próprio.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub kind: BlockState,
}

/// Marcador do caminho legado de blocos gerados individualmente.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedChunkBlock;

/// Marca a entidade renderizável que representa um chunk gerado.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedChunk {
    pub coord: ChunkCoord,
}
