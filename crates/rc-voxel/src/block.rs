/// Identificador compacto para um tipo de bloco registrado.
///
/// `BlockId` é a identidade de runtime de um tipo de bloco. Ele é
/// intencionalmente pequeno porque chunks armazenam muitos estados de bloco.
/// Nomes estáveis como `rustcraft:stone` ficam no registry, não dentro de cada
/// bloco do mundo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u16);

/// Valor concreto de bloco armazenado em uma posição do mundo voxel.
///
/// `id` seleciona a definição de bloco registrada. `variant` fica reservado
/// para estado finito, como orientação, aceso/apagado, idade de crescimento ou
/// formato de slab. Blocos simples de terreno usam a variante `0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockState {
    pub id: BlockId,
    pub variant: u16,
}

pub const AIR: BlockId = BlockId(0);
pub const GRASS: BlockId = BlockId(1);
pub const DIRT: BlockId = BlockId(2);
pub const STONE: BlockId = BlockId(3);

impl BlockState {
    /// Cria o estado padrão para um id de bloco.
    pub const fn new(id: BlockId) -> Self {
        Self { id, variant: 0 }
    }

    /// Retorna o estado canônico de bloco vazio.
    pub const fn air() -> Self {
        Self::new(AIR)
    }

    /// Retorna se este estado representa espaço vazio.
    pub fn is_air(self) -> bool {
        self.id == AIR
    }
}

#[cfg(test)]
mod tests {
    use super::{AIR, BlockState, GRASS};

    #[test]
    fn new_block_state_uses_default_variant() {
        let state = BlockState::new(GRASS);

        assert_eq!(state.id, GRASS);
        assert_eq!(state.variant, 0);
    }

    #[test]
    fn air_state_uses_air_id() {
        assert_eq!(BlockState::air().id, AIR);
        assert!(BlockState::air().is_air());
    }
}
