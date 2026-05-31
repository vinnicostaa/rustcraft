use bevy::prelude::Resource;

/// Configurações visuais simples do protótipo.
#[derive(Resource, Debug, Clone, Copy)]
pub struct RenderConfig {
    /// Tamanho de um bloco voxel em unidades de mundo Bevy.
    pub block_size: f32,
    /// Intensidade da luz solar direcional.
    pub sun_illuminance: f32,
    /// Habilita sombras na luz principal quando suportado pelo pipeline.
    pub shadows_enabled: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            block_size: 1.0,
            sun_illuminance: 10_000.0,
            shadows_enabled: true,
        }
    }
}
