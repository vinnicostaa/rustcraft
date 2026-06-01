use bevy::prelude::*;
use rc_voxel::{BlockState, STONE};

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedBlock {
    block: BlockState,
}

impl Default for SelectedBlock {
    fn default() -> Self {
        SelectedBlock {
            block: BlockState::new(STONE),
        }
    }
}

impl SelectedBlock {
    /// Retorna o bloco que será usado na próxima colocação.
    pub fn block(self) -> BlockState {
        self.block
    }

    pub(crate) fn set(&mut self, block: BlockState) {
        self.block = block;
    }
}

#[cfg(test)]
mod tests {
    use super::SelectedBlock;
    use rc_voxel::STONE;

    #[test]
    fn selected_block_defaults_to_stone() {
        assert_eq!(SelectedBlock::default().block().id, STONE);
    }
}
