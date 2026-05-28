# Arquitetura do rustcraft

Este documento registra a separação atual de responsabilidades do `rustcraft` e a direção técnica para evoluir o protótipo voxel sem transformar tudo em um único `main.rs`.

## Objetivo da separação

A arquitetura atual ainda é pequena, mas já cria fronteiras para os sistemas que tendem a crescer:

- input e controles;
- ações/intents de gameplay;
- player/câmera;
- geração e dados de mundo;
- blocos;
- renderização;
- configuração;
- UI/debug tools.

A prioridade é manter o código simples, mas com fronteiras reais de Cargo workspace para exercitar packages/crates, APIs públicas e dependências sem ciclos.

## Fluxo de runtime

```mermaid
graph TD
    A[Input bruto Bevy KeyCode] --> B[rc-input]
    B --> C[ActionState / PlayerAction]
    C --> D[rc-player]
    D --> E[Transform do Player/Camera]

    J[rc-voxel BlockType] --> I[rc-render BlockRenderAssets]
    J --> H[rc-world]
    I --> H
    H --> K[Entidades de blocos]
```

## Packages do workspace

| Package | Responsabilidade atual | Não deve assumir |
| --- | --- | --- |
| `rustcraft` | Bin/app principal: `DefaultPlugins`, `RustcraftPlugin` e composição dos plugins internos. | Regras de gameplay, dados voxel, render assets ou input físico. |
| `rc-input` | `PlayerAction`, `ActionState`, bindings teclado → ação e `InputPlugin`. | Mover player, gerar mundo ou conhecer render. |
| `rc-player` | `Player`, `PlayerConfig`, spawn da câmera/player e movimento por ações. | Ler `KeyCode` diretamente, gerar terreno ou criar materiais. |
| `rc-voxel` | `BlockType` e regras puras de bloco. | Depender de Bevy, meshes, materials, input ou player. |
| `rc-render` | `RenderConfig`, iluminação, mesh/material compartilhados e `BlockRenderAssets`. | Gerar topologia do mundo ou mapear controles. |
| `rc-world` | `WorldConfig`, `Block`, geração inicial e spawn dos blocos. | Mapear teclas, mover player ou decidir materiais. |

## Ordem dos sistemas

As crates que precisam de ordenação exportam seus próprios sets. A dependência relevante hoje é:

```text
rc-render::RenderStartupSet::PrepareAssets
    -> rc-world::spawn_initial_chunk
```

Isso garante que `rc-world` só use `BlockRenderAssets` depois que `rc-render` criou os handles de mesh/material.

Em runtime:

```text
PreUpdate / rc-input::InputSet::CollectInput
    ↓
Update / rc-player::move_player
```

O input é coletado em `PreUpdate`; o movimento consome `ActionState` em `Update`.

## Decisões atuais

### Bevy continua sendo o runtime principal

O projeto segue com Bevy porque o objetivo imediato é estudar ECS, plugins, renderização 3D, assets e sistemas de gameplay sem construir engine do zero.

### Workspace multi-crate didático

A estrutura agora implementa a ADR-0003 registrada no vault:

```text
crates/rustcraft   # bin/app principal
crates/rc-input    # input bruto -> ações semânticas
crates/rc-player   # player/câmera/controlador
crates/rc-voxel    # dados voxel puros
crates/rc-render   # luz, materiais, meshes, render plugin
crates/rc-world    # geração/spawn inicial do mundo
```

O grafo intencional é:

```text
rustcraft
├── rc-input
├── rc-player ──→ rc-input
├── rc-voxel
├── rc-render ──→ rc-voxel
└── rc-world  ──→ rc-voxel, rc-render
```

`rc-voxel` fica sem dependência de Bevy para manter a fronteira de domínio mais pura.

### Input bruto não move gameplay diretamente

`rc-input` traduz `KeyCode` para `PlayerAction`. `rc-player` consome `ActionState`. Essa separação facilita:

- remapeamento de teclas;
- suporte a gamepad;
- playback/replay;
- input de rede no futuro;
- testes de gameplay sem simular teclado.

### Bloco lógico é separado de render

`rc-voxel` define `BlockId`, `BlockState` e metadados de bloco; `rc-render` decide como transformar esses dados em material e mesh. Isso prepara o caminho para:

- textura/atlas;
- meshing por chunk;
- blocos com propriedades físicas;
- blocos invisíveis/técnicos;
- serialização de mundo sem carregar assets gráficos.

## Limitações conhecidas

O spawn principal já usa uma entidade renderizável para o chunk inicial, gerada a partir de `Chunk` + mesh com faces expostas.

O projeto já tem `Chunk` em memória, armazenamento compacto de blocos, geração de mesh por chunk com faces expostas e spawn inicial por chunk. As próximas etapas técnicas importantes são:

1. recuperar material/visual por tipo de bloco via atlas, vertex color ou abordagem equivalente;
2. atualização parcial de chunks alterados;
3. colliders por chunk, não por bloco individual;
4. consulta de chunks vizinhos para remover faces internas entre chunks.

## Próximas fronteiras recomendadas

1. Adicionar diagnosticos de chunk/render para medir entidades, faces e FPS.
2. Recuperar material por tipo de bloco sem voltar ao spawn por bloco.
3. Adicionar raycast/interação de bloco.
4. Integrar Rapier com collider por chunk.
5. Consultar chunks vizinhos no meshing para evitar faces internas entre chunks.
6. Adicionar `menu`/debug overlay para render distance, wireframe/diagnósticos e posição do player.
