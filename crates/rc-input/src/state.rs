use std::collections::HashSet;

use bevy::prelude::Resource;

use crate::PlayerAction;

/// Ações ativas no frame atual.
#[derive(Resource, Debug, Clone, Default)]
pub struct ActionState {
    active: HashSet<PlayerAction>,
}

impl ActionState {
    /// Limpa todas as ações acumuladas antes de coletar o input do frame.
    pub fn clear(&mut self) {
        self.active.clear();
    }

    /// Marca uma ação como ativa no frame atual.
    pub fn press(&mut self, action: PlayerAction) {
        self.active.insert(action);
    }

    /// Retorna se uma ação está ativa no frame atual.
    pub fn pressed(&self, action: PlayerAction) -> bool {
        self.active.contains(&action)
    }

    /// Itera pelas ações ativas no frame atual.
    pub fn iter(&self) -> impl Iterator<Item = PlayerAction> + '_ {
        self.active.iter().copied()
    }
}
