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

const BLOCK_HIT_EPSILON: f32 = 0.001;

/// Converte um hit de raycast em uma posição inteira de bloco.
///
/// O ponto do hit costuma ficar exatamente na face do bloco. Para descobrir o
/// bloco atingido, a função desloca o ponto levemente para dentro da face usando
/// a normal e só então converte coordenadas de mundo para coordenadas voxel.
pub fn block_pos_from_hit(point: [f32; 3], normal: [f32; 3], block_size: f32) -> Option<BlockPos> {
    if !block_size.is_finite() || block_size <= 0.0 {
        return None;
    }

    if !point.iter().all(|value| value.is_finite()) {
        return None;
    }

    if !normal.iter().all(|value| value.is_finite()) {
        return None;
    }

    let normal_length =
        (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();

    if normal_length <= f32::EPSILON {
        return None;
    }

    let nudge = block_size * BLOCK_HIT_EPSILON;
    let inside_point = [
        point[0] - normal[0] / normal_length * nudge,
        point[1] - normal[1] / normal_length * nudge,
        point[2] - normal[2] / normal_length * nudge,
    ];

    Some(BlockPos::new(
        world_coord_to_block(inside_point[0], block_size),
        world_coord_to_block(inside_point[1], block_size),
        world_coord_to_block(inside_point[2], block_size),
    ))
}

fn world_coord_to_block(coord: f32, block_size: f32) -> i32 {
    (coord / block_size).floor() as i32
}

#[cfg(test)]
mod tests {
    use super::{BlockPos, ChunkCoord, block_pos_from_hit};

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

    #[test]
    fn block_pos_from_hit_uses_inside_hit_face() {
        let pos = block_pos_from_hit([2.4, 3.0, 1.2], [0.0, 1.0, 0.0], 1.0);

        assert_eq!(pos, Some(BlockPos::new(2, 2, 1)));
    }

    #[test]
    fn block_pos_from_hit_handles_positive_x_face() {
        let pos = block_pos_from_hit([4.0, 1.2, 0.5], [1.0, 0.0, 0.0], 1.0);

        assert_eq!(pos, Some(BlockPos::new(3, 1, 0)));
    }

    #[test]
    fn block_pos_from_hit_handles_negative_coordinates() {
        let pos = block_pos_from_hit([-1.0, 2.2, 0.3], [-1.0, 0.0, 0.0], 1.0);

        assert_eq!(pos, Some(BlockPos::new(-1, 2, 0)));
    }

    #[test]
    fn block_pos_from_hit_rejects_invalid_block_size() {
        let pos = block_pos_from_hit([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], 0.0);

        assert_eq!(pos, None);
    }
}
