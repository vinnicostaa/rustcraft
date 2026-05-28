use bevy::prelude::{Entity, Resource};
use rc_voxel::{BlockPos, BlockState, Chunk, ChunkCoord};
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

    /// Converte posição global de bloco para coordenada de chunk.
    fn world_to_chunk_coord(pos: BlockPos, chunk_size: i32) -> Option<ChunkCoord> {
        if chunk_size <= 0 {
            return None;
        }

        Some(ChunkCoord::new(
            pos.x.div_euclid(chunk_size),
            pos.y.div_euclid(chunk_size),
            pos.z.div_euclid(chunk_size),
        ))
    }

    /// Altera um bloco por coordenada global e marca o chunk como dirty.
    ///
    /// Retorna `true` se a posição pertence a um chunk carregado e foi alterada.
    pub fn set_block(&mut self, pos: BlockPos, block: BlockState, chunk_size: i32) -> bool {
        let Some(coord) = Self::world_to_chunk_coord(pos, chunk_size) else {
            return false;
        };

        let Some(entry) = self.chunks.get_mut(&coord) else {
            return false;
        };

        let lx = pos.x.rem_euclid(chunk_size);
        let ly = pos.y.rem_euclid(chunk_size);
        let lz = pos.z.rem_euclid(chunk_size);

        let changed = entry.data.set(lx, ly, lz, block);

        if changed {
            entry.dirty = true;
        }
        changed
    }

    /// Itera sobre as coordenadas dos chunks marcados como dirty.
    pub fn dirty_coords(&self) -> impl Iterator<Item = ChunkCoord> + '_ {
        self.chunks.iter().filter(|(_, e)| e.dirty).map(|(c, _)| *c)
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

    #[test]
    fn set_block_marks_dirty_and_changes_block() {
        let mut map = ChunkMap::default();
        let coord = ChunkCoord::new(0, 0, 0);
        let chunk = Chunk::new_filled(4, BlockState::air());

        map.insert(coord, chunk, Entity::PLACEHOLDER);

        // Bloco global (1,1,1) → local (1,1,1) no chunk (0,0,0) com chunk_size=4
        let pos = BlockPos::new(1, 1, 1);
        assert!(map.set_block(pos, BlockState::new(rc_voxel::STONE), 4));

        let entry = map.get(coord).unwrap();
        assert!(entry.dirty);
        assert_eq!(
            entry.data.get(1, 1, 1),
            Some(BlockState::new(rc_voxel::STONE))
        );
    }

    #[test]
    fn set_block_returns_false_for_unloaded_chunk() {
        let mut map = ChunkMap::default();
        let pos = BlockPos::new(100, 0, 0);

        assert!(!map.set_block(pos, BlockState::air(), 16));
    }

    #[test]
    fn dirty_coords_lists_only_dirty_chunks() {
        let mut map = ChunkMap::default();
        let coord = ChunkCoord::new(0, 0, 0);
        let chunk = Chunk::new_filled(4, BlockState::air());

        map.insert(coord, chunk, Entity::PLACEHOLDER);

        assert_eq!(map.dirty_coords().count(), 0);

        map.mark_dirty(coord);

        let dirty: Vec<_> = map.dirty_coords().collect();
        assert_eq!(dirty, vec![coord]);
    }
}
