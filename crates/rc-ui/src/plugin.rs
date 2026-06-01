use bevy::prelude::*;

use crate::{
    crosshair::{hide_crosshair, show_crosshair, spawn_crosshair, update_crosshair_target},
    hotbar::{hide_hotbar, show_hotbar, spawn_hotbar, update_hotbar_selection},
};

use rc_interaction::InteractionSet;

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
        app.add_systems(Startup, (spawn_hotbar, spawn_crosshair))
            .add_systems(
                Update,
                update_hotbar_selection.run_if(in_state(self.visible_state.clone())),
            )
            .add_systems(
                PostUpdate,
                update_crosshair_target
                    .after(InteractionSet::UpdateAim)
                    .run_if(in_state(self.visible_state.clone())),
            )
            .add_systems(
                OnEnter(self.visible_state.clone()),
                (show_hotbar, show_crosshair),
            )
            .add_systems(
                OnEnter(self.hidden_state.clone()),
                (hide_hotbar, hide_crosshair),
            );
    }
}
