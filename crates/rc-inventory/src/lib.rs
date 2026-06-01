//! Seleção mínima de blocos e base para inventário do `rustcraft`.
//!
//! Este crate mantém o estado de gameplay relacionado ao bloco selecionado,
//! sem depender do app principal. A UI e a interação podem ler esse estado por
//! meio de `SelectedBlock`.

mod plugin;
mod selected;
mod slots;

pub use plugin::InventoryPlugin;
pub use selected::SelectedBlock;
pub use slots::{HOTBAR_SLOTS, HotbarSlotDefinition};
