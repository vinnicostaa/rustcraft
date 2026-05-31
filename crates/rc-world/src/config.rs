use bevy::prelude::Resource;

/// Seed usada para gerar o mesmo mundo de forma determinística.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldSeed(pub u64);

/// Configurações iniciais de geração do mundo.
#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldConfig {
    /// Seed determinística usada pelo gerador de terreno.
    pub seed: WorldSeed,
    /// Tamanho de cada chunk em blocos por eixo.
    pub chunk_size: i32,
    /// Altura máxima usada pela função protótipo de terreno.
    pub max_height: i32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            seed: WorldSeed(0),
            chunk_size: 24,
            max_height: 6,
        }
    }
}
