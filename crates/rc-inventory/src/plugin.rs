use bevy::prelude::*;

use crate::{
    selected::SelectedBlock,
    slots::{HOTBAR_SLOTS, block_for_hotbar_key},
};

/// Plugin da seleção mínima de bloco.
pub struct InventoryPlugin<S: States> {
    active_state: S,
}

impl<S: States> InventoryPlugin<S> {
    pub fn active_in(active_state: S) -> Self {
        Self { active_state }
    }
}

impl<S: States> Plugin for InventoryPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedBlock>().add_systems(
            Update,
            select_block_from_hotbar.run_if(in_state(self.active_state.clone())),
        );
    }
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
