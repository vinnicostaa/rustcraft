//! Biblioteca principal do `rustcraft`.
//!
//! Esta package é o app/binário principal. Ela compõe as crates internas
//! `rc-*` e mantém a cola de app que ainda depende do estado global, como
//! pausa, cursor e diagnósticos gerais.

pub mod app;
mod diagnostics;
mod state;

pub use app::run;
