//! HUD e interface de gameplay do `rustcraft`.
//!
//! A crate observa estado de gameplay exposto por outras crates, como
//! `rc-inventory`, e desenha a interface com Bevy UI. Ela não altera mundo nem
//! decide regras de inventário.

mod crosshair;
mod hotbar;
mod plugin;

pub use plugin::UiPlugin;
