use bevy::prelude::*;

use crate::{
    actions::apply_block_interaction, aimed_block::AimedBlock, gizmo::draw_aimed_block_gizmo,
    raycast::update_aimed_block,
};

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
        app.init_resource::<AimedBlock>().add_systems(
            PostUpdate,
            (
                update_aimed_block.after(TransformSystems::Propagate),
                apply_block_interaction.after(update_aimed_block),
                draw_aimed_block_gizmo.after(update_aimed_block),
            )
                .run_if(in_state(self.active_state.clone())),
        );
    }
}
