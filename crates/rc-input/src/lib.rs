use std::collections::HashSet;

use bevy::prelude::*;

/// Runtime set for systems that translate physical input into game actions.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSet {
    CollectInput,
}

/// Semantic actions consumed by gameplay systems.
///
/// Gameplay systems should not know about `KeyCode`; they read player intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

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

const KEY_BINDINGS: &[(KeyCode, PlayerAction)] = &[
    (KeyCode::KeyW, PlayerAction::MoveForward),
    (KeyCode::KeyS, PlayerAction::MoveBackward),
    (KeyCode::KeyA, PlayerAction::MoveLeft),
    (KeyCode::KeyD, PlayerAction::MoveRight),
    (KeyCode::Space, PlayerAction::MoveUp),
    (KeyCode::ShiftLeft, PlayerAction::MoveDown),
];

/// Translates raw Bevy keyboard input into semantic game actions.
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
