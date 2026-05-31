use bevy::prelude::*;

/// Configurações iniciais do controlador de câmera livre.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerConfig {
    /// Velocidade de voo usada pelo movimento livre.
    pub fly_speed: f32,
    /// Multiplicador aplicado ao movimento acumulado do mouse.
    pub mouse_sensitivity: f32,
    /// Posição inicial do player/câmera no mundo.
    pub spawn_position: Vec3,
    /// Ponto observado no spawn para calcular a rotação inicial.
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
