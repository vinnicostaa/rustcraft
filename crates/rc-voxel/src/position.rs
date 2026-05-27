/// Posição inteira de um bloco nas coordenadas voxel do mundo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Posição inteira de um chunk no grid de chunks.
///
/// Uma coordenada de chunk identifica uma região de blocos. Ela não é a mesma
/// coisa que uma coordenada de bloco; `origin_block_pos` converte a coordenada
/// para a primeira posição de bloco coberta por aquele chunk em um determinado
/// tamanho de chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
    use super::{BlockPos, ChunkCoord};

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

    #[test]
    fn chunk_coord_origin_block_pos_uses_chunk_size() {
        let coord = ChunkCoord::new(2, 0, -1);

        assert_eq!(coord.origin_block_pos(16), BlockPos::new(32, 0, -16));
    }
}
