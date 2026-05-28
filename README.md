# rustcraft

`rustcraft` é um projeto de estudo em Rust/Bevy para aprender e evoluir fundamentos de jogos 3D voxel: câmera, movimentação, geração de terreno, interface, itens, lógica de mundo, física, texturas, áudio e, futuramente, multiplayer.

A ideia é partir de uma base parecida com jogos sandbox/voxel, mas sem ficar preso a uma recriação 1:1 de Minecraft. O objetivo de longo prazo é experimentar sistemas mais abertos de sobrevivência, construção, automação e simulação.

## Estrutura do workspace

Este repositório usa Cargo workspace para deixar o projeto pronto para crescer sem misturar todas as responsabilidades em um único arquivo.

```text
.
├── Cargo.toml              # workspace root: membros, deps, lints e profiles
├── Cargo.lock              # lockfile compartilhado
├── AGENTS.md               # guia para agentes e tutores em novas sessões
├── CODING_PRACTICES.md     # práticas de codificação e documentação
├── ARCHITECTURE.md         # notas de arquitetura e responsabilidades
└── crates/
    ├── rustcraft/          # package/bin principal: compõe o app Bevy
    │   └── src/
    │       ├── app.rs
    │       ├── diagnostics.rs
    │       ├── lib.rs
    │       └── bin/
    │           └── rustcraft.rs
    ├── rc-input/           # input físico -> ações semânticas
    │   └── src/
    │       ├── actions.rs
    │       ├── bindings.rs
    │       ├── lib.rs
    │       ├── plugin.rs
    │       └── state.rs
    ├── rc-player/          # player/câmera/controlador
    │   └── src/
    │       ├── camera.rs
    │       ├── components.rs
    │       ├── config.rs
    │       ├── lib.rs
    │       ├── movement.rs
    │       └── plugin.rs
    ├── rc-voxel/           # dados voxel puros
    │   └── src/
    │       ├── block.rs
    │       ├── chunk.rs
    │       ├── generation.rs
    │       ├── lib.rs
    │       ├── position.rs
    │       └── registry.rs
    ├── rc-render/          # luz, mesh/material e assets visuais
    │   └── src/
    │       ├── assets.rs
    │       ├── config.rs
    │       ├── lib.rs
    │       ├── lighting.rs
    │       ├── materials.rs
    │       ├── meshing.rs
    │       └── plugin.rs
    └── rc-world/           # geração/spawn inicial do mundo
        └── src/
            ├── components.rs
            ├── config.rs
            ├── diagnostics.rs
            ├── generation.rs
            ├── lib.rs
            ├── plugin.rs
            └── spawn.rs
```

## Arquitetura atual

O fluxo principal agora é separado por camadas:

```text
teclado/winit/Bevy input
        ↓
rc-input: mapeia KeyCode para PlayerAction
        ↓
rc-input: ActionState com intenções de jogo
        ↓
rc-player: movimenta a câmera/player

rc-voxel ─→ rc-world ─→ rc-render
  dados       geração       assets/visual
  voxel       de chunk      Bevy
```

Essa separação segue a direção discutida na pesquisa de arquitetura:

- **Zed/workspace:** workspace com dependências centralizadas e packages por fronteira.
- **COSMIC/eventos:** input bruto separado de ações de alto nível.
- **Bevy:** composição por plugins, resources, components e schedules explícitos.
- **Voxel/Rapier futuro:** mundo e player já estão separados para introduzir chunks, colisão e character controller depois.

Mais detalhes em [`ARCHITECTURE.md`](ARCHITECTURE.md), [`CODING_PRACTICES.md`](CODING_PRACTICES.md) e [`AGENTS.md`](AGENTS.md).

## Comandos

Na raiz do repositório:

```sh
cargo run
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Estado atual

O protótipo já gera o mundo inicial como dados de `Chunk`, transforma esse chunk em um `Mesh` Bevy com apenas faces expostas e spawna uma entidade renderizável para o chunk inicial.

Esse caminho remove o custo estrutural de uma entidade renderizável por bloco comum de terreno. A renderização ainda é temporária visualmente: o chunk usa um material único até existir atlas de textura, vertex color ou outro mecanismo de material por face/bloco.

Implementado:

- workspace Cargo;
- package principal em `crates/rustcraft`;
- library crates internas `rc-input`, `rc-player`, `rc-voxel`, `rc-render` e `rc-world`;
- crates internas organizadas por módulos de domínio, com `lib.rs` como API pública;
- plugin raiz do jogo compondo plugins das crates internas;
- camada de input semântico separada de teclado físico;
- câmera/player com movimento WASD + Space/Shift;
- modelo de blocos baseado em `BlockId`, `BlockState`, `BlockDefinition` e registry mínimo;
- `Chunk` puro em `rc-voxel`, armazenando `BlockState` sem depender de Bevy;
- geração determinística com `WorldSeed` e `TerrainGenerator`;
- geração inicial de chunk em `rc-world::generate_chunk`;
- geração de mesh por chunk em `rc-render::build_chunk_mesh_data` e `rc-render::build_chunk_mesh`;
- spawn inicial com uma entidade renderizável por chunk;
- diagnósticos de runtime com FPS, frame time, contagem de entidades, CPU, memória, chunks, faces e vértices;
- assets compartilhados para mesh/material de blocos em crate render.

Limitações atuais:

- o chunk inicial ainda usa material único temporário;
- diagnósticos próprios ainda cobrem apenas o chunk inicial;
- ainda não há greedy meshing, atlas de textura ou culling próprio por chunk;
- a função de terreno usa uma fórmula simples com seno/cosseno e seed; noise procedural real entra depois.

## Roadmap inicial

1. Base jogável: câmera, movimentação, geração simples e UI básica.
2. Mundo voxel: chunks, dados de bloco, mesh por chunk e faces expostas.
3. Gameplay básico: itens, seleção/interação com blocos, inventário simples.
4. Sistemas técnicos: física, textura/atlas, áudio e debug overlay.
5. Escala: geração assíncrona, culling, render distance, configuração e profiling.
6. Futuro: multiplayer, modding/extensões e ferramentas de criação.
