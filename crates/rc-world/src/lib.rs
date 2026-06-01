//! Geração, spawn e estado carregado do mundo voxel.
//!
//! Esta crate coordena seed/configuração de mundo, geração de `Chunk`, registro
//! de chunks carregados e remesh de chunks dirty. Dados voxel continuam em
//! `rc-voxel`; construção visual da mesh continua em `rc-render`.

mod chunk_map;
mod components;
mod config;
mod diagnostics;
mod generation;
mod plugin;
mod remesh;
mod spawn;

pub use chunk_map::{ChunkEntry, ChunkMap};
pub use components::GeneratedChunk;
pub use config::{WorldConfig, WorldSeed};
pub use generation::{TerrainGenerator, generate_chunk, terrain_height};
pub use plugin::WorldPlugin;
