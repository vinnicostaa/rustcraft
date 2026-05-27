use bevy::prelude::*;

/// Initial fly-camera controller settings.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PlayerConfig {
    pub fly_speed: f32,
    pub spawn_position: Vec3,
    pub look_at: Vec3,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            fly_speed: 10.0,
            spawn_position: Vec3::new(8.0, 5.0, 20.0),
            look_at: Vec3::new(8.0, 0.0, 8.0),
        }
    }
}
