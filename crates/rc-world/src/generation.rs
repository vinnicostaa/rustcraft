use crate::WorldSeed;
use rc_voxel::{BlockState, Chunk, ChunkCoord, block_for_layer};

/// Gerador determinístico de terreno.
#[derive(Debug, Clone, Copy)]
pub struct TerrainGenerator {
    seed: WorldSeed,
    max_height: i32,
}

/// Função protótipo simples para altura do terreno.
///
/// Retorna uma altura entre `0` e `altura_máxima`.
pub fn terrain_height(x: i32, z: i32, max_height: i32) -> i32 {
    terrain_height_with_seed(x, z, max_height, WorldSeed(0))
}

fn terrain_height_with_seed(x: i32, z: i32, max_height: i32, seed: WorldSeed) -> i32 {
    if max_height <= 0 {
        return 0;
    }

    let seed_offset = seed.0 as f32 * 0.001;

    let fx = x as f32 * 0.3 + seed_offset;
    let fz = z as f32 * 0.3 + seed_offset;

    let noise = (fx.sin() + fz.cos() + (fx * 0.5).cos()) / 3.0;
    let normalized = (noise + 1.0) / 2.0;

    (normalized * max_height as f32) as i32
}

pub fn generate_chunk(coord: ChunkCoord, chunk_size: i32, generator: &TerrainGenerator) -> Chunk {
    let mut chunk = Chunk::new_filled(chunk_size, BlockState::air());
    let origin = coord.origin_block_pos(chunk_size);

    for local_x in 0..chunk_size {
        for local_z in 0..chunk_size {
            let global_x = origin.x + local_x;
            let global_z = origin.z + local_z;
            let surface_y = generator.height_at(global_x, global_z);

            for local_y in 0..chunk_size {
                let global_y = origin.y + local_y;

                if global_y <= surface_y {
                    let block = block_for_layer(global_y, surface_y);

                    chunk.set(local_x, local_y, local_z, block);
                }
            }
        }
    }
    chunk
}

impl TerrainGenerator {
    pub const fn new(seed: WorldSeed, max_height: i32) -> Self {
        Self { seed, max_height }
    }

    pub fn height_at(&self, x: i32, z: i32) -> i32 {
        terrain_height_with_seed(x, z, self.max_height, self.seed)
    }
}

#[cfg(test)]
mod tests {
    use super::{TerrainGenerator, generate_chunk, terrain_height};
    use crate::WorldSeed;
    use rc_voxel::ChunkCoord;

    #[test]
    fn terrain_height_stays_within_expected_range() {
        let max_height = 6;

        for x in -16..16 {
            for z in -16..16 {
                let height = terrain_height(x, z, max_height);
                assert!(
                    (0..=max_height).contains(&height),
                    "height {height} was outside 0..={max_height} at {x},{z}"
                );
            }
        }
    }

    #[test]
    fn terrain_height_handles_non_positive_max_height() {
        assert_eq!(terrain_height(0, 0, 0), 0);
        assert_eq!(terrain_height(0, 0, -10), 0);
    }

    #[test]
    fn generate_chunk_uses_requested_size() {
        let generator = TerrainGenerator::new(WorldSeed(42), 6);
        let chunk = generate_chunk(ChunkCoord::new(0, 0, 0), 16, &generator);

        assert_eq!(chunk.size(), 16);
    }

    #[test]
    fn generated_chunk_contains_terrain_blocks() {
        let generator = TerrainGenerator::new(WorldSeed(42), 6);
        let chunk = generate_chunk(ChunkCoord::new(0, 0, 0), 16, &generator);

        let mut found_non_air = false;

        for x in 0..chunk.size() {
            for y in 0..chunk.size() {
                for z in 0..chunk.size() {
                    if let Some(block) = chunk.get(x, y, z)
                        && !block.is_air()
                    {
                        found_non_air = true;
                    }
                }
            }
        }

        assert!(found_non_air);
    }

    // TerrainGenerator
    #[test]
    fn same_seed_generates_same_height() {
        let a = TerrainGenerator::new(WorldSeed(42), 6);
        let b = TerrainGenerator::new(WorldSeed(42), 6);

        assert_eq!(a.height_at(10, 20), b.height_at(10, 20));
    }

    #[test]
    fn different_seeds_can_generate_different_height() {
        let a = TerrainGenerator::new(WorldSeed(1), 6);
        let b = TerrainGenerator::new(WorldSeed(999), 6);

        assert_ne!(a.height_at(10, 20), b.height_at(10, 20));
    }

    #[test]
    fn zero_seed_generator_matches_terrain_height() {
        let generator = TerrainGenerator::new(WorldSeed(0), 6);

        for x in -16..16 {
            for z in -16..16 {
                assert_eq!(generator.height_at(x, z), terrain_height(x, z, 6));
            }
        }
    }
}
