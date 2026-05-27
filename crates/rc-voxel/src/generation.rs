use crate::{BlockState, DIRT, GRASS, STONE};

/// Seleciona o bloco de terreno padrão para uma camada vertical.
///
/// Esta é intencionalmente uma pequena regra de geração de mundo, não uma regra
/// do registry de blocos. O registry descreve o que os blocos são; a geração
/// decide onde eles aparecem.
pub fn block_for_layer(y: i32, surface_y: i32) -> BlockState {
    let depth = surface_y - y;

    match depth {
        0 => BlockState::new(GRASS),
        1..=2 => BlockState::new(DIRT),
        _ => BlockState::new(STONE),
    }
}

#[cfg(test)]
mod tests {
    use super::block_for_layer;
    use crate::{DIRT, GRASS, STONE};

    #[test]
    fn layer_mapping_uses_grass_on_surface() {
        assert_eq!(block_for_layer(5, 5).id, GRASS);
    }

    #[test]
    fn layer_mapping_uses_dirt_near_surface() {
        assert_eq!(block_for_layer(4, 5).id, DIRT);
        assert_eq!(block_for_layer(3, 5).id, DIRT);
    }

    #[test]
    fn layer_mapping_uses_stone_deeper_down() {
        assert_eq!(block_for_layer(2, 5).id, STONE);
    }
}
