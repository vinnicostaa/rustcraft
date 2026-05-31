use bevy::prelude::*;

use crate::{ActionState, bindings::KEY_BINDINGS};

/// Set de runtime para sistemas que traduzem input físico em ações de jogo.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSet {
    /// Coleta input bruto e atualiza `ActionState`.
    CollectInput,
}

/// Traduz input bruto de teclado do Bevy em ações semânticas.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionState>()
            .configure_sets(PreUpdate, InputSet::CollectInput)
            .add_systems(
                PreUpdate,
                collect_keyboard_actions.in_set(InputSet::CollectInput),
            );
    }
}

fn collect_keyboard_actions(keys: Res<ButtonInput<KeyCode>>, mut actions: ResMut<ActionState>) {
    actions.clear();

    for &(key, action) in KEY_BINDINGS {
        if keys.pressed(key) {
            actions.press(action);
        }
    }
}
