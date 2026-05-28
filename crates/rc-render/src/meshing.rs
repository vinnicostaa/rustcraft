//! Geração de mesh para chunks voxel.
//!
//! Este módulo é intencionalmente independente do Bevy ECS: recebe um `Chunk`
//! puro e produz `ChunkMeshData`, uma struct de dados que pode ser convertida
//! em `bevy::mesh::Mesh` via `Into<Mesh>`.
//!
//! Separar a geração de dados da construção do `Mesh` Bevy facilita testes
//! unitários sem depender do runtime Bevy e permite mover o meshing para uma
//! thread separada no futuro.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;
use rc_voxel::Chunk;

const VERTICES_PER_FACE: usize = 4;
const INDICES_PER_FACE: usize = 6;

const FACE_UVS: [[f32; 2]; VERTICES_PER_FACE] = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

const FACE_INDICES: [u32; INDICES_PER_FACE] = [0, 1, 2, 0, 2, 3];

// ---------------------------------------------------------------------------
// Tipos públicos
// ---------------------------------------------------------------------------

/// Dados brutos de vértices e índices para a mesh de um chunk.
///
/// Contém apenas dados Rust puros — sem handles Bevy, sem `Assets`, sem ECS.
/// Use `Into<Mesh>` para converter em um `bevy::mesh::Mesh` pronto para spawn.
///
/// # Invariantes
///
/// - `positions`, `normals` e `uvs` têm sempre o mesmo comprimento.
/// - `indices.len()` é sempre múltiplo de 6 (duas faces triangulares por quad).
#[derive(Debug, Default, Clone)]
pub struct ChunkMeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// UVs por vértice. Placeholder por enquanto: cada face usa coordenadas
    /// `[0,1]²`. Quando atlas de textura entrar, este campo passará a
    /// referenciar a região correta do atlas por tipo de bloco e face.
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl ChunkMeshData {
    /// Número de faces trianguladas na mesh (cada face = 2 triângulos = 6 índices).
    pub fn face_count(&self) -> usize {
        self.debug_assert_valid();
        self.indices.len() / INDICES_PER_FACE
    }

    /// Retorna `true` se não há geometria gerada.
    pub fn is_empty(&self) -> bool {
        debug_assert_eq!(
            self.positions.is_empty(),
            self.indices.is_empty(),
            "positions e indices devem ficar vazios juntos"
        );
        self.positions.is_empty()
    }

    /// Converte estes dados em um `Mesh` Bevy.
    ///
    /// Este método existe por legibilidade no call site. O mesmo resultado pode
    /// ser obtido com `Mesh::from(data)` ou `data.into()`.
    pub fn into_mesh(self) -> Mesh {
        self.into()
    }

    fn debug_assert_valid(&self) {
        debug_assert_eq!(
            self.positions.len(),
            self.normals.len(),
            "positions e normals devem ter o mesmo comprimento"
        );
        debug_assert_eq!(
            self.positions.len(),
            self.uvs.len(),
            "positions e uvs devem ter o mesmo comprimento"
        );
        debug_assert!(
            self.indices.len().is_multiple_of(INDICES_PER_FACE),
            "indices.len() deve ser múltiplo de {INDICES_PER_FACE}; encontrado {}",
            self.indices.len()
        );
    }
}

/// Converte `ChunkMeshData` em um `bevy::mesh::Mesh` pronto para uso no ECS.
///
/// O `Mesh` resultante usa `PrimitiveTopology::TriangleList` e
/// `RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD`, o mesmo
/// padrão usado nos exemplos oficiais do Bevy para meshes editáveis.
impl From<ChunkMeshData> for Mesh {
    fn from(data: ChunkMeshData) -> Self {
        data.debug_assert_valid();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            editable_mesh_asset_usages(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, data.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs)
        .with_inserted_indices(Indices::U32(data.indices))
    }
}

fn editable_mesh_asset_usages() -> RenderAssetUsages {
    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
}

// ---------------------------------------------------------------------------
// Definição de faces
// ---------------------------------------------------------------------------

/// Descreve uma das 6 faces de um cubo unitário.
///
/// `neighbor` é o deslocamento inteiro para o bloco vizinho nessa direção.
/// `normal` é o vetor normal da face (unitário, aponta para fora).
/// `corners` são os 4 vértices da face em coordenadas locais do bloco `[0,1]³`,
/// em ordem anti-horária vista de fora (winding correto para back-face culling).
#[derive(Debug, Clone, Copy)]
struct FaceDefinition {
    neighbor: [i32; 3],
    normal: [f32; 3],
    corners: [[f32; 3]; 4],
}

/// As 6 faces de um cubo unitário, em ordem: +X, -X, +Y, -Y, +Z, -Z.
const FACES: [FaceDefinition; 6] = [
    // +X (direita)
    FaceDefinition {
        neighbor: [1, 0, 0],
        normal: [1.0, 0.0, 0.0],
        corners: [
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
        ],
    },
    // -X (esquerda)
    FaceDefinition {
        neighbor: [-1, 0, 0],
        normal: [-1.0, 0.0, 0.0],
        corners: [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
    },
    // +Y (topo)
    FaceDefinition {
        neighbor: [0, 1, 0],
        normal: [0.0, 1.0, 0.0],
        corners: [
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
    },
    // -Y (base)
    FaceDefinition {
        neighbor: [0, -1, 0],
        normal: [0.0, -1.0, 0.0],
        corners: [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    },
    // +Z (frente)
    FaceDefinition {
        neighbor: [0, 0, 1],
        normal: [0.0, 0.0, 1.0],
        corners: [
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
    },
    // -Z (trás)
    FaceDefinition {
        neighbor: [0, 0, -1],
        normal: [0.0, 0.0, -1.0],
        corners: [
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
    },
];

// ---------------------------------------------------------------------------
// Geração de mesh
// ---------------------------------------------------------------------------

/// Gera um `Mesh` Bevy para um chunk inteiro.
///
/// Esta é a função de conveniência para os sistemas de renderização. Para testes
/// e etapas que precisam inspecionar os buffers antes de criar o asset Bevy, use
/// [`build_chunk_mesh_data`].
pub fn build_chunk_mesh(chunk: &Chunk, block_size: f32) -> Mesh {
    build_chunk_mesh_data(chunk, block_size).into_mesh()
}

/// Gera os dados de mesh para um chunk inteiro, emitindo apenas faces expostas.
///
/// Uma face é considerada exposta quando o bloco vizinho naquela direção é ar
/// ou está fora dos limites do chunk. Faces internas entre dois blocos sólidos
/// não são emitidas.
///
/// `block_size` escala as coordenadas dos vértices: use `1.0` para blocos de
/// 1 metro, `0.5` para blocos de meio metro, etc.
pub fn build_chunk_mesh_data(chunk: &Chunk, block_size: f32) -> ChunkMeshData {
    // Estimativa conservadora de capacidade para evitar realocações frequentes.
    // Um chunk 16³ com terreno típico expõe ~30% das faces; 6 faces × 4 vértices
    // por bloco sólido é o pior caso. Começar com 25% do pior caso é razoável.
    let estimated_faces = (chunk.size() * chunk.size() * chunk.size() / 4) as usize;
    let estimated_vertices = estimated_faces * VERTICES_PER_FACE;

    let mut data = ChunkMeshData {
        positions: Vec::with_capacity(estimated_vertices),
        normals: Vec::with_capacity(estimated_vertices),
        uvs: Vec::with_capacity(estimated_vertices),
        indices: Vec::with_capacity(estimated_faces * INDICES_PER_FACE),
    };

    for x in 0..chunk.size() {
        for y in 0..chunk.size() {
            for z in 0..chunk.size() {
                let Some(block) = chunk.get(x, y, z) else {
                    continue;
                };

                if block.is_air() {
                    continue;
                }

                for face in FACES {
                    let nx = x + face.neighbor[0];
                    let ny = y + face.neighbor[1];
                    let nz = z + face.neighbor[2];

                    // Exposta se o vizinho for ar ou estiver fora do chunk.
                    let exposed = match chunk.get(nx, ny, nz) {
                        Some(neighbor) => neighbor.is_air(),
                        None => true,
                    };

                    if exposed {
                        push_face(&mut data, x, y, z, block_size, face);
                    }
                }
            }
        }
    }

    data
}

/// Adiciona os 4 vértices e 2 triângulos de uma face ao buffer de mesh.
fn push_face(
    data: &mut ChunkMeshData,
    x: i32,
    y: i32,
    z: i32,
    block_size: f32,
    face: FaceDefinition,
) {
    let base = data.positions.len() as u32;

    let ox = x as f32;
    let oy = y as f32;
    let oz = z as f32;

    for corner in face.corners {
        data.positions.push([
            (ox + corner[0]) * block_size,
            (oy + corner[1]) * block_size,
            (oz + corner[2]) * block_size,
        ]);
        data.normals.push(face.normal);
    }

    // Placeholder até atlas de textura entrar: cada face ocupa o tile inteiro.
    data.uvs.extend_from_slice(&FACE_UVS);

    // Dois triângulos por quad: (0,1,2) e (0,2,3).
    data.indices.extend(FACE_INDICES.map(|index| base + index));
}

// ---------------------------------------------------------------------------
// Testes
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use bevy::asset::RenderAssetUsages;
    use bevy::mesh::{Indices, Mesh};
    use bevy::render::render_resource::PrimitiveTopology;
    use rc_voxel::{BlockState, Chunk, STONE};

    use super::{build_chunk_mesh, build_chunk_mesh_data};

    /// Chunk vazio não deve gerar nenhuma geometria.
    #[test]
    fn empty_chunk_generates_empty_mesh() {
        let chunk = Chunk::new_filled(2, BlockState::air());

        let mesh = build_chunk_mesh_data(&chunk, 1.0);

        assert!(mesh.is_empty());
        assert_eq!(mesh.face_count(), 0);
    }

    /// Um bloco isolado no centro de um chunk deve expor todas as 6 faces.
    #[test]
    fn single_block_generates_six_faces() {
        let mut chunk = Chunk::new_filled(3, BlockState::air());
        chunk.set(1, 1, 1, BlockState::new(STONE));

        let mesh = build_chunk_mesh_data(&chunk, 1.0);

        assert_eq!(mesh.face_count(), 6);
        assert_eq!(mesh.positions.len(), 24); // 6 faces × 4 vértices
        assert_eq!(mesh.normals.len(), 24);
        assert_eq!(mesh.uvs.len(), 24);
        assert_eq!(mesh.indices.len(), 36); // 6 faces × 2 triângulos × 3 índices
    }

    /// Dois blocos adjacentes não devem gerar a face compartilhada entre eles.
    ///
    /// Dois cubos isolados teriam 12 faces. Encostados devem ter 10
    /// (a face +X do primeiro e a face -X do segundo são internas).
    #[test]
    fn adjacent_blocks_do_not_generate_internal_faces() {
        let mut chunk = Chunk::new_filled(3, BlockState::air());
        chunk.set(1, 1, 1, BlockState::new(STONE));
        chunk.set(2, 1, 1, BlockState::new(STONE));

        let mesh = build_chunk_mesh_data(&chunk, 1.0);

        assert_eq!(mesh.face_count(), 10);
    }

    /// Bloco na borda do chunk deve expor a face que aponta para fora.
    #[test]
    fn block_at_chunk_border_exposes_outward_face() {
        let mut chunk = Chunk::new_filled(2, BlockState::air());
        chunk.set(0, 0, 0, BlockState::new(STONE));

        let mesh = build_chunk_mesh_data(&chunk, 1.0);

        // Bloco no canto (0,0,0) de um chunk 2³: 3 faces internas (vizinhos
        // sólidos em +X, +Y, +Z se existirem) e 3 faces externas. Como o
        // chunk está vazio exceto por este bloco, todos os 6 vizinhos são ar
        // ou fora do chunk — 6 faces expostas.
        assert_eq!(mesh.face_count(), 6);
    }

    /// `block_size` deve escalar as posições dos vértices corretamente.
    #[test]
    fn block_size_scales_vertex_positions() {
        let mut chunk = Chunk::new_filled(2, BlockState::air());
        chunk.set(0, 0, 0, BlockState::new(STONE));

        let mesh_1 = build_chunk_mesh_data(&chunk, 1.0);
        let mesh_2 = build_chunk_mesh_data(&chunk, 2.0);

        // Todos os vértices de mesh_2 devem ser exatamente o dobro de mesh_1.
        for (v1, v2) in mesh_1.positions.iter().zip(mesh_2.positions.iter()) {
            assert!((v2[0] - v1[0] * 2.0).abs() < f32::EPSILON);
            assert!((v2[1] - v1[1] * 2.0).abs() < f32::EPSILON);
            assert!((v2[2] - v1[2] * 2.0).abs() < f32::EPSILON);
        }
    }

    /// `positions`, `normals` e `uvs` devem ter sempre o mesmo comprimento.
    #[test]
    fn vertex_buffers_have_consistent_lengths() {
        let mut chunk = Chunk::new_filled(4, BlockState::air());
        chunk.set(1, 1, 1, BlockState::new(STONE));
        chunk.set(2, 1, 1, BlockState::new(STONE));
        chunk.set(1, 2, 1, BlockState::new(STONE));

        let mesh = build_chunk_mesh_data(&chunk, 1.0);

        assert_eq!(mesh.positions.len(), mesh.normals.len());
        assert_eq!(mesh.positions.len(), mesh.uvs.len());
    }

    /// A conversão final deve produzir um `Mesh` compatível com renderização 3D.
    #[test]
    fn build_chunk_mesh_returns_renderable_bevy_mesh() {
        let mut chunk = Chunk::new_filled(3, BlockState::air());
        chunk.set(1, 1, 1, BlockState::new(STONE));

        let mesh = build_chunk_mesh(&chunk, 1.0);

        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
        assert_eq!(
            mesh.asset_usage,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
        );
        assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());

        let indices = mesh.indices().expect("mesh deve ter índice");
        assert!(matches!(indices, Indices::U32(_)));
        assert_eq!(indices.len(), 36);
    }
}
