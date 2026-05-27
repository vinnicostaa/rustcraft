use bevy::prelude::KeyCode;

use crate::PlayerAction;

pub(crate) const KEY_BINDINGS: &[(KeyCode, PlayerAction)] = &[
    (KeyCode::KeyW, PlayerAction::MoveForward),
    (KeyCode::KeyS, PlayerAction::MoveBackward),
    (KeyCode::KeyA, PlayerAction::MoveLeft),
    (KeyCode::KeyD, PlayerAction::MoveRight),
    (KeyCode::Space, PlayerAction::MoveUp),
    (KeyCode::ShiftLeft, PlayerAction::MoveDown),
];
