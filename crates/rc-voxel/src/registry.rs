use crate::{AIR, BlockId, DIRT, GRASS, STONE};

/// Propriedades estáticas compartilhadas por todo estado de bloco com o mesmo id.
///
/// Chunks armazenam valores compactos de `BlockState`. Sistemas que precisam
/// de informação semântica, como colisão ou renderização, consultam a definição
/// correspondente em vez de duplicar essas propriedades em cada bloco.
#[derive(Debug, Clone, Copy)]
pub struct BlockDefinition {
    pub id: BlockId,
    pub key: &'static str,
    pub solid: bool,
    pub opaque: bool,
    pub hardness: f32,
}

/// Consulta a definição built-in para um id de bloco.
///
/// Esta é a primeira forma, estática, de um registry de blocos. Ela mantém a
/// decisão de catálogo centralizada enquanto o projeto ainda é pequeno; depois
/// pode ser substituída por um registry baseado em resource ou asset sem mudar
/// os dados armazenados nos chunks.
pub fn block_definition(id: BlockId) -> Option<BlockDefinition> {
    match id {
        AIR => Some(BlockDefinition {
            id: AIR,
            key: "rustcraft:air",
            solid: false,
            opaque: false,
            hardness: 0.0,
        }),
        GRASS => Some(BlockDefinition {
            id: GRASS,
            key: "rustcraft:grass",
            solid: true,
            opaque: true,
            hardness: 0.6,
        }),
        DIRT => Some(BlockDefinition {
            id: DIRT,
            key: "rustcraft:dirt",
            solid: true,
            opaque: true,
            hardness: 0.5,
        }),
        STONE => Some(BlockDefinition {
            id: STONE,
            key: "rustcraft:stone",
            solid: true,
            opaque: true,
            hardness: 1.5,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::block_definition;
    use crate::{AIR, BlockId, STONE};

    #[test]
    fn air_definition_is_not_solid_or_opaque() {
        let definition = block_definition(AIR).expect("air should be registered");

        assert!(!definition.solid);
        assert!(!definition.opaque);
        assert_eq!(definition.key, "rustcraft:air");
    }

    #[test]
    fn terrain_block_definition_is_solid_and_opaque() {
        let definition = block_definition(STONE).expect("stone should be registered");

        assert!(definition.solid);
        assert!(definition.opaque);
        assert_eq!(definition.key, "rustcraft:stone");
    }

    #[test]
    fn unknown_block_id_has_no_definition() {
        assert!(block_definition(BlockId(999)).is_none());
    }
}
