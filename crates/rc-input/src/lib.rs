//! Entrada física traduzida para ações semânticas de gameplay.
//!
//! Esta crate é a fronteira entre APIs de input do Bevy, como `KeyCode`, e o
//! restante do jogo. Sistemas de player e gameplay consomem `PlayerAction` e
//! `ActionState`, evitando acoplamento direto a teclado ou janela.

mod actions;
mod bindings;
mod plugin;
mod state;

pub use actions::PlayerAction;
pub use plugin::{InputPlugin, InputSet};
pub use state::ActionState;
