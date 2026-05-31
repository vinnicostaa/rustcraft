//! Player, câmera e controlador de movimento.
//!
//! Esta crate consome ações semânticas de `rc-input` e atualiza o `Transform`
//! do player/câmera. Ela não conhece `KeyCode` nem `GameState`; o app principal
//! controla quando o player está habilitado por meio de `PlayerControlState`.

mod camera;
mod components;
mod config;
mod look;
mod movement;
mod plugin;

pub use components::Player;
pub use config::PlayerConfig;
pub use plugin::{PlayerControlState, PlayerPlugin};
