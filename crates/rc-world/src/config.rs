use bevy::prelude::Resource;

/// Seed usada para gerar o mesmo mundo de forma deterministica
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldSeed(pub u64);

/// Configurações iniciais de geração do mundo.
#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldConfig {
    pub seed: WorldSeed,
    pub chunk_size: i32,
    pub max_height: i32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: WorldSeed(42),
            chunk_size: 24,
            max_height: 6,
        }
    }
}
