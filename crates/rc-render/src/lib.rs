//! Renderização voxel e recursos visuais Bevy.
//!
//! Esta crate transforma dados puros de `rc-voxel`, como `Chunk` e
//! `BlockState`, em meshes, materiais e recursos Bevy. Ela não gera terreno nem
//! decide estado de mundo.

mod assets;
mod config;
mod lighting;
mod materials;
mod meshing;
mod plugin;

pub use assets::ChunkRenderAssets;
pub use config::RenderConfig;
pub use meshing::{ChunkMeshData, build_chunk_mesh, build_chunk_mesh_data};
pub use plugin::{RenderPlugin, RenderStartupSet};
