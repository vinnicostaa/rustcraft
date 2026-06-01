use bevy::prelude::*;
use rc_voxel::BlockPos;

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AimedBlock {
    hit: Option<AimedBlockHit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AimedBlockHit {
    pub target_pos: BlockPos,
    pub adjacent_pos: BlockPos,
}

impl AimedBlock {
    pub(crate) fn clear(&mut self) {
        self.hit = None;
    }

    pub(crate) fn set(&mut self, hit: AimedBlockHit) {
        self.hit = Some(hit);
    }

    pub fn hit(&self) -> Option<AimedBlockHit> {
        self.hit
    }
}

#[cfg(test)]
mod tests {
    use super::{AimedBlock, AimedBlockHit};
    use rc_voxel::BlockPos;

    #[test]
    fn aimed_block_defaults_to_no_hit() {
        assert_eq!(AimedBlock::default().hit(), None);
    }

    #[test]
    fn aimed_block_stores_hit() {
        let hit = AimedBlockHit {
            target_pos: BlockPos::new(1, 2, 3),
            adjacent_pos: BlockPos::new(1, 2, 4),
        };

        let mut aimed_block = AimedBlock::default();
        aimed_block.set(hit);

        assert_eq!(aimed_block.hit(), Some(hit));
    }

    #[test]
    fn aimed_block_clear_removes_hit() {
        let mut aimed_block = AimedBlock::default();

        aimed_block.set(AimedBlockHit {
            target_pos: BlockPos::new(1, 2, 3),
            adjacent_pos: BlockPos::new(1, 2, 4),
        });

        aimed_block.clear();

        assert_eq!(aimed_block.hit(), None);
    }
}
