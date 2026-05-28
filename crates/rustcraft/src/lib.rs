//! Biblioteca principal do `rustcraft`.
//!
//! Esta package é o app/binário principal. Regras de jogo e recursos de
//! domínio vivem nas crates internas `rc-*`.

pub mod app;
mod diagnostics;
mod interaction;

pub use app::run;
