//! Interação do player com blocos do mundo.
//!
//! Esta crate mantém raycast de bloco, estado do bloco mirado e ações mínimas
//! de quebrar/colocar bloco. Ela consome player, render, mundo e inventário sem
//! depender do app principal.

mod actions;
mod aimed_block;
mod gizmo;
mod plugin;
mod raycast;

pub use aimed_block::{AimedBlock, AimedBlockHit};
pub use plugin::{InteractionPlugin, InteractionSet};
