use bevy::prelude::*;
use rc_voxel::{BlockState, DIRT, GRASS, STONE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotbarSlotDefinition {
    pub(crate) key: KeyCode,
    pub slot: u8,
    pub block: BlockState,
}

pub const HOTBAR_SLOTS: [HotbarSlotDefinition; 3] = [
    HotbarSlotDefinition {
        key: KeyCode::Digit1,
        slot: 1,
        block: BlockState::new(GRASS),
    },
    HotbarSlotDefinition {
        key: KeyCode::Digit2,
        slot: 2,
        block: BlockState::new(DIRT),
    },
    HotbarSlotDefinition {
        key: KeyCode::Digit3,
        slot: 3,
        block: BlockState::new(STONE),
    },
];

/// Converte teclas numéricas da hotbar mínima em blocos colocáveis.
pub(crate) fn block_for_hotbar_key(key: KeyCode) -> Option<BlockState> {
    HOTBAR_SLOTS
        .iter()
        .find(|slot| slot.key == key)
        .map(|slot| slot.block)
}

#[cfg(test)]
mod tests {
    use super::block_for_hotbar_key;
    use bevy::prelude::KeyCode;
    use rc_voxel::{DIRT, GRASS, STONE};

    #[test]
    fn digit_1_selects_grass() {
        let block = block_for_hotbar_key(KeyCode::Digit1);

        assert_eq!(block.map(|block| block.id), Some(GRASS));
    }

    #[test]
    fn digit_2_selects_dirt() {
        let block = block_for_hotbar_key(KeyCode::Digit2);

        assert_eq!(block.map(|block| block.id), Some(DIRT));
    }

    #[test]
    fn digit_3_selects_stone() {
        let block = block_for_hotbar_key(KeyCode::Digit3);

        assert_eq!(block.map(|block| block.id), Some(STONE));
    }

    #[test]
    fn unrelated_key_does_not_select_block() {
        assert_eq!(block_for_hotbar_key(KeyCode::KeyQ), None);
    }
}
