use bevy::prelude::Resource;

/// Simple visual settings for the prototype.
#[derive(Resource, Debug, Clone, Copy)]
pub struct RenderConfig {
    pub block_size: f32,
    pub sun_illuminance: f32,
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
