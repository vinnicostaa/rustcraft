use crate::BlockState;

/// Região cúbica de blocos armazenada de forma compacta.
///
/// O chunk guarda os blocos em um vetor linear (`Vec<BlockState>`), mas expõe
/// acesso por coordenadas locais `x`, `y` e `z`. Essas coordenadas são locais ao
/// chunk: `0..size` em cada eixo.
pub struct Chunk {
    size: i32,
    blocks: Vec<BlockState>,
}

impl Chunk {
    /// Cria um chunk cúbico preenchido com o mesmo estado de bloco.
    ///
    /// Esta função é útil para construir chunks vazios (`BlockState::air()`) ou
    /// cenários simples de teste. O tamanho precisa ser positivo.
    pub fn new_filled(size: i32, block: BlockState) -> Self {
        assert!(size > 0, "chunk size must be positive");

        let volume = (size * size * size) as usize;

        Self {
            size,
            blocks: vec![block; volume],
        }
    }

    /// Retorna o bloco em uma posição local do chunk.
    ///
    /// Coordenadas fora do intervalo `0..size` retornam `None`.
    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<BlockState> {
        let index = self.index(x, y, z)?;

        Some(self.blocks[index])
    }

    /// Altera o bloco em uma posição local do chunk.
    ///
    /// Retorna `true` quando a posição existe e foi alterada. Retorna `false`
    /// quando qualquer coordenada está fora do chunk.
    pub fn set(&mut self, x: i32, y: i32, z: i32, block: BlockState) -> bool {
        let Some(index) = self.index(x, y, z) else {
            return false;
        };

        self.blocks[index] = block;
        true
    }

    /// Converte coordenadas locais `x`, `y`, `z` em índice do vetor interno.
    fn index(&self, x: i32, y: i32, z: i32) -> Option<usize> {
        if x < 0 || y < 0 || z < 0 {
            return None;
        }

        if x >= self.size || y >= self.size || z >= self.size {
            return None;
        }

        Some((x + y * self.size + z * self.size * self.size) as usize)
    }

    /// Retorna o tamanho do chunk em cada eixo.
    pub fn size(&self) -> i32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::Chunk;
    use crate::{BlockState, STONE};

    #[test]
    fn new_chunk_is_filled_with_block() {
        let chunk = Chunk::new_filled(2, BlockState::air());

        assert_eq!(chunk.get(0, 0, 0), Some(BlockState::air()));
        assert_eq!(chunk.get(1, 1, 1), Some(BlockState::air()));
    }

    #[test]
    fn set_changes_block_at_position() {
        let mut chunk = Chunk::new_filled(2, BlockState::air());
        let stone = BlockState::new(STONE);

        assert!(chunk.set(1, 0, 1, stone));
        assert_eq!(chunk.get(1, 0, 1), Some(stone))
    }

    #[test]
    fn out_of_bounds_coordinates_fail() {
        let mut chunk = Chunk::new_filled(2, BlockState::air());

        assert_eq!(chunk.get(2, 0, 0), None);
        assert!(!chunk.set(-1, 0, 0, BlockState::air()));
    }

    #[test]
    fn chunk_exposes_size() {
        let chunk = Chunk::new_filled(16, BlockState::air());

        assert_eq!(chunk.size(), 16);
    }
}
