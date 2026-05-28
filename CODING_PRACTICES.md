# Praticas de codificacao

Este documento registra o padrao de desenvolvimento do `rustcraft`.

## Principios

- Priorize codigo simples, testavel e coerente com o desenho atual.
- Separe dominio puro de runtime Bevy sempre que isso reduzir acoplamento.
- Prefira uma mudanca pequena e validada a um refactor grande sem necessidade imediata.
- Nao altere comportamento de jogo em refactors estruturais, salvo quando a task pedir isso explicitamente.
- Quando uma limitacao tecnica for aceita temporariamente, documente a limitacao e a proxima etapa.

## Organizacao Rust

- `lib.rs` deve expor a API publica do crate e delegar implementacao para modulos internos.
- Modulos devem representar responsabilidade: `config`, `components`, `plugin`, `generation`, `spawn`, `meshing`, `materials`.
- Tipos de dominio devem ficar perto da regra que representam.
- Use `pub(crate)` para APIs internas do crate.
- Use `pub` apenas quando outro crate realmente precisa consumir a API.
- Evite dependencias circulares entre crates.

## Fronteiras de crates

- `rc-voxel` nao deve depender de Bevy. Ele guarda `BlockId`, `BlockState`, `Chunk`, coordenadas e regras puras.
- `rc-world` gera dados de mundo e decide quando spawnar entidades.
- `rc-render` transforma dados em recursos visuais Bevy: `Mesh`, materiais, luz e assets.
- `rc-player` consome acoes de jogo, nao `KeyCode` diretamente.
- `rc-input` conhece teclado/controle e produz intencoes de gameplay.
- `rustcraft` compoe plugins; nao deve acumular regra de gameplay.

## Bevy

- Use plugins para instalar sistemas, resources e startup/setup.
- Quando houver dependencia de ordem, exporte `SystemSet` nomeado pelo crate dono.
- Para mesh customizada em Bevy 0.18.1, siga o padrao:
  - `Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)`;
  - atributos `Mesh::ATTRIBUTE_POSITION`, `Mesh::ATTRIBUTE_NORMAL`, `Mesh::ATTRIBUTE_UV_0`;
  - indices `Indices::U32`.
- Para render 3D, use o padrao atual do Bevy: `Mesh3d(handle)` + `MeshMaterial3d(material)`.
- Nao crie entidade Bevy para cada bloco comum de terreno no modelo final. Bloco comum e dado de chunk.

## Voxel

- `BlockState` deve permanecer compacto: `id` + `variant`.
- Dados grandes ou dinamicos de bloco devem ir para estrutura auxiliar ligada a `BlockPos`, nao para cada celula do chunk.
- Meshing de chunk deve emitir apenas faces visiveis.
- Faces entre blocos solidos do mesmo chunk nao devem ser geradas.
- Faces na borda do chunk podem ser consideradas expostas ate existir consulta de chunks vizinhos.
- Materiais por bloco devem evoluir para atlas/vertex color/render layers, nao para voltar ao spawn por bloco.

## Documentacao

- Documentos do projeto devem estar em pt_BR.
- Comentarios de codigo devem explicar intencao ou invariantes, nao narrar linha por linha.
- Documente limitacoes conhecidas no README/ARCHITECTURE quando elas afetam a proxima task.
- Atualize o vault quando uma decisao, divida tecnica ou estado de sprint mudar.
- Use audit log datado para mudancas estruturais relevantes.

## Testes e validacao

- Toda regra pura nova deve ter teste unitario.
- Mudanca em API publica de crate deve rodar testes do crate e do workspace.
- Gates padrao:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Fluxo didatico

Quando a task for para o usuario codar:

1. Explique o objetivo em uma frase.
2. Mostre os arquivos alvo.
3. Explique a sintaxe Rust nova que sera usada.
4. Mostre um exemplo pequeno de antes/depois quando ajudar.
5. Defina criterios de pronto.
6. Depois que o usuario terminar, revise antes de sugerir nova task.
