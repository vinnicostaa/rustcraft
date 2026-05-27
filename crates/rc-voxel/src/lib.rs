/// Tipo de bloco lógico.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
}

/// BlockPos representa uma posição inteira no grid voxel:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Posição inteira de um chunk no grid de chunks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockType {
    /// Seleciona um tipo de bloco com base na distância da superfície.
    pub fn for_layer(y: i32, surface_y: i32) -> Self {
        let depth = surface_y - y;

        match depth {
            0 => Self::Grass,
            1..=2 => Self::Dirt,
            _ => Self::Stone,
        }
    }

    /// Retorna se este bloco ocupa espaço físico.
    pub fn is_solid(self) -> bool {
        match self {
            Self::Air => false,
            Self::Grass | Self::Dirt | Self::Stone => true,
        }
    }
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn above(self) -> Self {
        Self {
            y: self.y + 1,
            ..self
        }
    }

    pub fn below(self) -> Self {
        Self {
            y: self.y - 1,
            ..self
        }
    }
}

impl ChunkCoord {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn origin_block_pos(self, chunk_size: i32) -> BlockPos {
        BlockPos::new(
            self.x * chunk_size,
            self.y * chunk_size,
            self.z * chunk_size,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockPos, BlockType, ChunkCoord};

    #[test]
    fn layer_mapping_uses_grass_on_surface() {
        assert_eq!(BlockType::for_layer(5, 5), BlockType::Grass);
    }

    #[test]
    fn layer_mapping_uses_dirt_near_surface() {
        assert_eq!(BlockType::for_layer(4, 5), BlockType::Dirt);
        assert_eq!(BlockType::for_layer(3, 5), BlockType::Dirt);
    }

    #[test]
    fn layer_mapping_uses_stone_deeper_down() {
        assert_eq!(BlockType::for_layer(2, 5), BlockType::Stone);
    }

    #[test]
    fn natural_blocks_are_solid() {
        assert!(BlockType::Grass.is_solid());
        assert!(BlockType::Dirt.is_solid());
        assert!(BlockType::Stone.is_solid());
    }

    #[test]
    fn air_is_not_solid() {
        assert!(!BlockType::Air.is_solid());
    }

    // BlockPos
    #[test]
    fn block_pos_new_stores_coordinates() {
        let pos = BlockPos::new(1, 2, 3);

        assert_eq!(pos.x, 1);
        assert_eq!(pos.y, 2);
        assert_eq!(pos.z, 3);
    }

    #[test]
    fn block_pos_above_and_below_change_only_y() {
        let pos = BlockPos::new(10, 20, 30);

        assert_eq!(pos.above(), BlockPos::new(10, 21, 30));
        assert_eq!(pos.below(), BlockPos::new(10, 19, 30));
    }

    // ChunkCoord
    #[test]
    fn chunk_coord_origin_block_pos_uses_chunk_size() {
        let coord = ChunkCoord::new(2, 0, -1);

        assert_eq!(coord.origin_block_pos(16), BlockPos::new(32, 0, -16));
    }
}
