use bevy::prelude::Resource;

/// Configurações iniciais de geração do mundo.
#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldConfig {
    pub chunk_size: i32,
    pub max_height: i32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            chunk_size: 24,
            max_height: 6,
        }
    }
}
