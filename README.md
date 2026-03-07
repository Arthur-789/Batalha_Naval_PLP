# Batalha Naval (Godot + Rust)

Este projeto usa Godot com GDExtension em Rust.

## Estrutura de `src/`

A pasta `src/` está organizada por camadas:

- `lib.rs`: ponto de entrada da biblioteca Rust e registro da extensão Godot.
- `application/`: orquestração de fluxo de jogo (input + coordenação).
- `domain/`: regras de negócio e estado do jogo (sem dependência de UI Godot).
- `presentation/`: lógica de cena/UI no Godot.

### `src/lib.rs`

- Declara os módulos principais:
  - `application`
  - `domain`
  - `presentation`
- Registra a extensão com `#[gdextension]`.

### `src/application/`

- `mod.rs`: índice dos módulos da camada de aplicação.
- `controlador_batalha.rs`:
  - Classe Godot `ControladorBatalha`.
  - Captura clique do mouse no `CampoIA`.
  - Executa disparo via domínio.
  - Atualiza tiles visuais com base no resultado do disparo.

### `src/domain/`

- `mod.rs`: índice dos módulos de domínio.
- `tabuleiro.rs`:
  - Define `BOARD_SIZE`.
  - Define `EstadoTabuleiro` e operações de estado (posicionar navio e leitura/escrita de célula).
- `disparo.rs`:
  - Define `ResultadoDisparo`.
  - Define `RetornoDisparo { resultado, mensagem }`.
  - Implementa `executar_disparo(...)` com regra de tiro e mensagem de feedback.
- `jogador.rs`:
  - Define `TipoJogador` e `Jogador` com estado comum (tabuleiro e regras básicas).
- `jogador_ia.rs`:
  - Define `JogadorIA` e `EstrategiaIA` para comportamento da IA por estratégia.

### `src/presentation/`

- `mod.rs`: índice dos módulos de apresentação.
- `batalha/`:
  - Feature de apresentação da batalha.
  - `renderizacao_tabuleiro/`: renderização de tiles, preview e mapeamento visual dos navios.
- `legacy/`:
  - Arquivos antigos de apresentação que não fazem parte do fluxo ativo.

## Fluxo atual (resumo)

1. O usuário clica no tabuleiro inimigo.
2. `ControladorBatalha` converte clique em coordenada de célula.
3. O domínio (`executar_disparo`) retorna resultado + mensagem.
4. O controller imprime a mensagem e atualiza o tile correspondente.
