use bevy::prelude::*;

pub(crate) fn block_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        unlit: false,
        ..default()
    }
}
