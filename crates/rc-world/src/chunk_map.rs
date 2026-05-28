use bevy::prelude::{Entity, Resource};
use rc_voxel::{Chunk, ChunkCoord};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ChunkMap {
    chunks: HashMap<ChunkCoord, ChunkEntry>,
}

pub struct ChunkEntry {
    pub data: Chunk,
    pub entity: Entity,
    pub dirty: bool,
}

impl ChunkMap {
    pub fn insert(&mut self, coord: ChunkCoord, data: Chunk, entity: Entity) {
        self.chunks.insert(
            coord,
            ChunkEntry {
                data,
                entity,
                dirty: false,
            },
        );
    }

    pub fn get(&self, coord: ChunkCoord) -> Option<&ChunkEntry> {
        self.chunks.get(&coord)
    }

    pub fn get_mut(&mut self, coord: ChunkCoord) -> Option<&mut ChunkEntry> {
        self.chunks.get_mut(&coord)
    }

    pub fn mark_dirty(&mut self, coord: ChunkCoord) -> bool {
        let Some(entry) = self.chunks.get_mut(&coord) else {
            return false;
        };

        entry.dirty = true;
        true
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rc_voxel::BlockState;

    #[test]
    fn new_chunk_map_is_empty() {
        let map = ChunkMap::default();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn insert_stores_chunk_entry() {
        let mut map = ChunkMap::default();
        let coord = ChunkCoord::new(0, 0, 0);
        let chunk = Chunk::new_filled(2, BlockState::air());

        map.insert(coord, chunk, Entity::PLACEHOLDER);

        let entry = map.get(coord).expect("chunk should exists");

        assert_eq!(entry.entity, Entity::PLACEHOLDER);
        assert!(!entry.dirty);
        assert_eq!(entry.data.size(), 2);
    }

    #[test]
    fn mark_dirty_changes_existing_entry() {
        let mut map = ChunkMap::default();
        let coord = ChunkCoord::new(0, 0, 0);
        let chunk = Chunk::new_filled(2, BlockState::air());

        map.insert(coord, chunk, Entity::PLACEHOLDER);

        assert!(map.mark_dirty(coord));
        assert!(map.get(coord).unwrap().dirty);
    }

    #[test]
    fn mark_dirty_returns_false_for_missing_chunk() {
        let mut map = ChunkMap::default();

        assert!(!map.mark_dirty(ChunkCoord::new(99, 0, 0)));
    }
}
