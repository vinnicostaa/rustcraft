use bevy::prelude::*;

use crate::{
    actions::apply_block_interaction, aimed_block::AimedBlock, gizmo::draw_aimed_block_gizmo,
    raycast::update_aimed_block,
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionSet {
    UpdateAim,
    ApplyActions,
    DrawDebug,
}

/// Sistemas de interação entre o player e o mundo.
pub struct InteractionPlugin<S: States> {
    active_state: S,
}

impl<S: States> InteractionPlugin<S> {
    pub fn active_in(active_state: S) -> Self {
        Self { active_state }
    }
}

impl<S: States> Plugin for InteractionPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<AimedBlock>()
            .configure_sets(
                PostUpdate,
                InteractionSet::UpdateAim.after(TransformSystems::Propagate),
            )
            .configure_sets(
                PostUpdate,
                InteractionSet::ApplyActions.after(InteractionSet::UpdateAim),
            )
            .configure_sets(
                PostUpdate,
                InteractionSet::DrawDebug.after(InteractionSet::UpdateAim),
            )
            .add_systems(
                PostUpdate,
                (
                    update_aimed_block.in_set(InteractionSet::UpdateAim),
                    apply_block_interaction.in_set(InteractionSet::ApplyActions),
                    draw_aimed_block_gizmo.in_set(InteractionSet::DrawDebug),
                )
                    .run_if(in_state(self.active_state.clone())),
            );
    }
}
