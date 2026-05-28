use bevy::prelude::*;

/// Configuracoes iniciais do controlador de camera livre.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerConfig {
    pub fly_speed: f32,
    pub mouse_sensitivity: f32,
    pub spawn_position: Vec3,
    pub look_at: Vec3,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            fly_speed: 10.0,
            mouse_sensitivity: 0.003,
            spawn_position: Vec3::new(8.0, 5.0, 20.0),
            look_at: Vec3::new(8.0, 0.0, 8.0),
        }
    }
}
