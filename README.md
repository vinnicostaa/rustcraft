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
    │       ├── state.rs
    │       └── bin/
    │           └── rustcraft.rs
    ├── rc-input/           # input físico -> ações semânticas
    │   └── src/
    │       ├── actions.rs
    │       ├── bindings.rs
    │       ├── lib.rs
    │       ├── plugin.rs
    │       └── state.rs
    ├── rc-interaction/     # raycast, bloco mirado e ações de bloco
    │   └── src/
    │       ├── actions.rs
    │       ├── aimed_block.rs
    │       ├── gizmo.rs
    │       ├── plugin.rs
    │       ├── raycast.rs
    │       └── lib.rs
    ├── rc-inventory/       # seleção mínima de bloco e base de inventário
    │   └── src/
    │       ├── lib.rs
    │       ├── plugin.rs
    │       ├── selected.rs
    │       └── slots.rs
    ├── rc-ui/              # HUD e UI de gameplay
    │   └── src/
    │       ├── crosshair.rs
    │       ├── hotbar.rs
    │       ├── lib.rs
    │       └── plugin.rs
    ├── rc-player/          # player/câmera/controlador
    │   └── src/
    │       ├── camera.rs
    │       ├── components.rs
    │       ├── config.rs
    │       ├── lib.rs
    │       ├── look.rs
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
            ├── chunk_map.rs
            ├── components.rs
            ├── config.rs
            ├── diagnostics.rs
            ├── generation.rs
            ├── lib.rs
            ├── plugin.rs
            ├── remesh.rs
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
rc-player: gira e movimenta a câmera/player

rustcraft: GameState InGame/Paused
        ↓
captura/libera cursor e informa estados ativos aos plugins

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

Esse caminho remove o custo estrutural de uma entidade renderizável por bloco comum de terreno. A renderização agora usa uma mesh agregada por chunk com vertex colors por tipo de bloco; atlas de textura ou outro mecanismo de material por face/bloco ficam como próxima evolução visual.

Implementado:

- workspace Cargo;
- package principal em `crates/rustcraft`;
- library crates internas `rc-input`, `rc-interaction`, `rc-inventory`, `rc-player`, `rc-voxel`, `rc-render`, `rc-ui` e `rc-world`;
- crates internas organizadas por módulos de domínio, com `lib.rs` como API pública;
- plugin raiz do jogo compondo plugins das crates internas;
- camada de input semântico separada de teclado físico;
- seleção mínima de bloco isolada em `rc-inventory`, com slots `1`/`2`/`3` para grama, terra e pedra;
- interação de bloco isolada em `rc-interaction`, com raycast, `AimedBlock`, quebra e colocação mínima;
- HUD de gameplay isolado em `rc-ui`, com hotbar visual lendo `rc-inventory::SelectedBlock` e mira/crosshair lendo `rc-interaction::AimedBlock`;
- `GameState` mínimo (`InGame`/`Paused`) controlando captura/liberação de cursor e sistemas de interação;
- `PlayerControlState` em `rc-player` para pausar mouse look/movimento sem acoplar o player ao estado do app;
- câmera/player com mouse look e movimento WASD + Space/Shift relativo à direção atual;
- modelo de blocos baseado em `BlockId`, `BlockState`, `BlockDefinition` e registry mínimo;
- `Chunk` puro em `rc-voxel`, armazenando `BlockState` sem depender de Bevy;
- `ChunkMap` em `rc-world`, ligando `ChunkCoord` a dados de chunk, entidade renderizável e flag dirty;
- geração determinística com `WorldSeed` e `TerrainGenerator`;
- geração inicial de chunk em `rc-world::generate_chunk`;
- geração de mesh por chunk em `rc-render::build_chunk_mesh_data` e `rc-render::build_chunk_mesh`;
- vertex colors por tipo de bloco na mesh do chunk, distinguindo grama, terra e pedra sem voltar ao spawn por bloco;
- spawn inicial com uma entidade renderizável por chunk;
- rebuild de mesh para chunks dirty depois de alteração de bloco;
- diagnósticos de runtime com FPS, frame time, contagem de entidades, CPU, memória, chunks, faces e vértices;
- raycast de interação a partir da câmera/player, com conversão para `BlockPos`, estado `AimedBlock` e highlight debug do bloco mirado;
- quebra de bloco com clique esquerdo, alterando o dado do chunk e reconstruindo a mesh;
- hotbar visual mínima com Bevy UI nativo, mostrando slots `1`/`2`/`3` e destaque do bloco selecionado;
- mira/crosshair visual mínima com Bevy UI nativo, mudando cor quando há bloco mirado;
- colocação mínima de bloco com clique direito, usando o bloco selecionado e marcando o chunk dirty;
- assets compartilhados de render por chunk em `rc-render`, sem manter o caminho legado de entidade/material por bloco comum.

Limitações atuais:

- vertex colors ainda são uma etapa visual temporária; atlas de textura ou array texture entram depois;
- ainda não há inventário por item, quantidades ou itens reais;
- o pause ainda não tem UI própria; `Esc` alterna diretamente entre gameplay e pausa;
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
