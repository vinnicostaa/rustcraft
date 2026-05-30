use bevy::prelude::*;
use rc_voxel::{BlockState, DIRT, GRASS, STONE};

use crate::state::GameState;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectedBlock {
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
    pub(crate) fn block(self) -> BlockState {
        self.block
    }

    fn set(&mut self, block: BlockState) {
        self.block = block;
    }
}

fn block_for_hotbar_key(key: KeyCode) -> Option<BlockState> {
    match key {
        KeyCode::Digit1 => Some(BlockState::new(GRASS)),
        KeyCode::Digit2 => Some(BlockState::new(DIRT)),
        KeyCode::Digit3 => Some(BlockState::new(STONE)),
        _ => None,
    }
}

fn select_block_from_hotbar(
    keys: Res<ButtonInput<KeyCode>>,
    mut selected_block: ResMut<SelectedBlock>,
) {
    for key in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3] {
        if keys.just_pressed(key)
            && let Some(block) = block_for_hotbar_key(key)
        {
            selected_block.set(block);
        }
    }
}

pub(crate) struct RustcraftSelectionPlugin;

impl Plugin for RustcraftSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBlock>().add_systems(
            Update,
            select_block_from_hotbar.run_if(in_state(GameState::InGame)),
        );
    }
}
