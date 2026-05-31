//! Biblioteca principal do `rustcraft`.
//!
//! Esta package é o app/binário principal. Ela compõe as crates internas
//! `rc-*` e mantém a cola de gameplay que ainda depende do estado do app, como
//! pausa, interação, seleção mínima de bloco e hotbar visual.

pub mod app;
mod diagnostics;
mod hotbar;
mod interaction;
mod selection;
mod state;

pub use app::run;
