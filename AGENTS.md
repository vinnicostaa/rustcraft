# AGENTS.md

Guia rapido para agentes trabalhando no `rustcraft`.

## Contrato principal

- O usuario esta aprendendo Rust/Bevy e prefere codar. Nao implemente tasks de codigo sem pedido explicito na sessao corrente.
- Quando o usuario pedir ajuda de implementacao, explique o antes/depois, a sintaxe Rust envolvida e o motivo da mudanca.
- Revisoes devem priorizar bugs, riscos estruturais, regressao de comportamento e testes faltando.
- Documentacao do projeto deve ficar em pt_BR.
- Commits, quando solicitados, devem seguir Conventional Commits: `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`.

## Leitura inicial

Antes de propor ou alterar codigo, leia:

1. `README.md`
2. `ARCHITECTURE.md`
3. `CODING_PRACTICES.md`
4. `crates/*/src/lib.rs`
5. Vault do projeto, quando disponivel:
   - `50-projects/rustcraft/CLAUDE.md`
   - `50-projects/rustcraft/STATE.md`
   - `50-projects/rustcraft/05-tasks/bevy-example-study-tasks.md`
   - `50-projects/rustcraft/10-schedule/sprint-capability-map.md`

## Pesquisa e fontes

- Use documentacao oficial e implementacoes locais instaladas antes de tratar algo como verdade tecnica.
- Para Bevy, o projeto usa `bevy = 0.18.1`; confira exemplos oficiais locais em `.cargo/registry` ou docs oficiais equivalentes.
- Se uma API puder ter mudado, confirme antes de orientar a task.
- Registre no vault decisoes ou mudancas estruturais relevantes.

## Fluxo de trabalho

1. Entender o estado real com `git status`, `rg` e leitura dos arquivos alvo.
2. Separar o que e bug, o que e melhoria e o que e decisao arquitetural.
3. Se o usuario for codar, entregar uma task pequena com arquivos alvo, passos e criterios de validacao.
4. Se o agente for autorizado a editar, manter a mudanca pequena e aderente aos padroes existentes.
5. Validar com os gates adequados.
6. Atualizar README, ARCHITECTURE e vault quando a mudanca alterar o estado do projeto.

## Gates padrao

Na raiz do workspace:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Para mudanca localizada, rode tambem o crate especifico, por exemplo:

```sh
cargo test -p rc-render
cargo clippy -p rc-render --all-targets --all-features -- -D warnings
```

## Fronteiras atuais

- `rustcraft`: composicao do app Bevy.
- `rc-input`: input fisico para acoes semanticas.
- `rc-player`: player, camera e movimento.
- `rc-voxel`: dados voxel puros, sem Bevy.
- `rc-render`: assets, materiais, iluminacao e mesh Bevy.
- `rc-world`: configuracao, geracao e spawn do mundo.

## Estado tecnico atual

- O mundo ja e gerado como `Chunk`.
- `rc-render` ja tem `build_chunk_mesh_data` e `build_chunk_mesh`.
- O spawn principal ainda usa uma entidade renderizavel por bloco.
- A proxima task de codigo deve trocar o spawn para usar a mesh por chunk, mantendo a limitacao documentada de material unico ate atlas/vertex color.
