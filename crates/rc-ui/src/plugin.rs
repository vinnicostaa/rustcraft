use bevy::prelude::*;

use crate::hotbar::{hide_hotbar, show_hotbar, spawn_hotbar, update_hotbar_selection};

/// Plugin de UI de gameplay.
pub struct UiPlugin<S: States> {
    visible_state: S,
    hidden_state: S,
}

impl<S: States> UiPlugin<S> {
    pub fn visible_in(visible_state: S, hidden_state: S) -> Self {
        Self {
            visible_state,
            hidden_state,
        }
    }
}

impl<S: States> Plugin for UiPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hotbar)
            .add_systems(
                Update,
                update_hotbar_selection.run_if(in_state(self.visible_state.clone())),
            )
            .add_systems(OnEnter(self.visible_state.clone()), show_hotbar)
            .add_systems(OnEnter(self.hidden_state.clone()), hide_hotbar);
    }
}
