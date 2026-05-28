//! Tipos centrais de dados voxel do `rustcraft`.
//!
//! Esta crate fica intencionalmente independente do Bevy. Ela modela blocos,
//! metadados de blocos, posições e pequenas regras de geração como dados Rust
//! puros, para que geração de mundo, renderização, física e persistência usem o
//! mesmo vocabulário estável.

mod block;
mod chunk;
mod generation;
mod position;
mod registry;

pub use block::{AIR, BlockId, BlockState, DIRT, GRASS, STONE};
pub use chunk::Chunk;
pub use generation::block_for_layer;
pub use position::{BlockPos, ChunkCoord};
pub use registry::{BlockDefinition, block_definition};
