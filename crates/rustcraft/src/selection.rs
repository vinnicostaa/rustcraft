use bevy::prelude::*;
use rc_voxel::{BlockState, DIRT, GRASS, STONE};

use crate::state::GameState;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectedBlock {
    block: BlockState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HotbarSlotDefinition {
    pub(crate) key: KeyCode,
    pub(crate) slot: u8,
    pub(crate) block: BlockState,
}

pub(crate) const HOTBAR_SLOTS: [HotbarSlotDefinition; 3] = [
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

impl Default for SelectedBlock {
    fn default() -> Self {
        SelectedBlock {
            block: BlockState::new(STONE),
        }
    }
}

impl SelectedBlock {
    /// Retorna o bloco que será usado na próxima colocação.
    pub(crate) fn block(self) -> BlockState {
        self.block
    }

    fn set(&mut self, block: BlockState) {
        self.block = block;
    }
}

/// Converte teclas numéricas da hotbar mínima em blocos colocáveis.
fn block_for_hotbar_key(key: KeyCode) -> Option<BlockState> {
    HOTBAR_SLOTS
        .iter()
        .find(|slot| slot.key == key)
        .map(|slot| slot.block)
}

/// Atualiza o bloco selecionado quando o jogador pressiona `1`, `2` ou `3`.
fn select_block_from_hotbar(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected_block: ResMut<SelectedBlock>,
) {
    for slot in HOTBAR_SLOTS {
        if keys.just_pressed(slot.key)
            && let Some(block) = block_for_hotbar_key(slot.key)
        {
            selected_block.set(block);
        }
    }
}

/// Plugin da seleção mínima de bloco.
pub(crate) struct RustcraftSelectionPlugin;

impl Plugin for RustcraftSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBlock>().add_systems(
            Update,
            select_block_from_hotbar.run_if(in_state(GameState::InGame)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectedBlock, block_for_hotbar_key};
    use bevy::prelude::KeyCode;
    use rc_voxel::{DIRT, GRASS, STONE};

    #[test]
    fn selected_block_defaults_to_stone() {
        assert_eq!(SelectedBlock::default().block().id, STONE);
    }

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
