use bevy::prelude::Component;
use rc_voxel::ChunkCoord;

/// Marca a entidade renderizável que representa um chunk gerado.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedChunk {
    pub coord: ChunkCoord,
}
