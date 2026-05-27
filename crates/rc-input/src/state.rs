use std::collections::HashSet;

use bevy::prelude::Resource;

use crate::PlayerAction;

/// Actions active in the current frame.
#[derive(Resource, Debug, Clone, Default)]
pub struct ActionState {
    active: HashSet<PlayerAction>,
}

impl ActionState {
    pub fn clear(&mut self) {
        self.active.clear();
    }

    pub fn press(&mut self, action: PlayerAction) {
        self.active.insert(action);
    }

    pub fn pressed(&self, action: PlayerAction) -> bool {
        self.active.contains(&action)
    }

    pub fn iter(&self) -> impl Iterator<Item = PlayerAction> + '_ {
        self.active.iter().copied()
    }
}
