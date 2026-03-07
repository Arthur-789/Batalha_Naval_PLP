
use godot::classes::{INode2D, Input, InputEvent, InputEventMouseButton, Label, Node, Node2D, TileMapLayer};
use godot::global::MouseButton;
use godot::prelude::*;

use crate::application::fase_posicionamento::FasePosicionamento;
use crate::application::helpers::{coordenadas, cursor};
use crate::application::gerenciador_turnos::{GerenciadorTurnos, EstadoTurno};
use crate::application::ias::ia_simples::IASimples;
use crate::domain::disparo::{ResultadoDisparo, executar_disparo};
use crate::domain::jogador::Jogador;
use crate::domain::jogador_ia::JogadorIA;
use crate::domain::tabuleiro::BOARD_SIZE;
use crate::presentation::batalha::{
    limpar_preview, render_preview_posicionamento, render_resultado_disparo, render_tabuleiro_jogador,
};
use crate::domain::tabuleiro::{Celula, BOARD_SIZE, EstadoTabuleiro};

const DELAY_TURNO_IA: f64 = 0.7;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FaseJogo {
    PosicionandoJogador,
    TurnoJogador,
    TurnoIAAguardandoDelay,
    FimDeJogo,
}

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct ControladorBatalha {
    // Campos do HEAD (antigo)
    jogador_humano: Jogador,
    jogador_ia: JogadorIA,
    fase_posicionamento: FasePosicionamento,
    fase: FaseJogo,
    tempo_restante_ia: f64,
    tooltip_instrucao: Option<Gd<Label>>,

    // Campos do branch novo
    tabuleiro_jogador: EstadoTabuleiro,
    tabuleiro_inimigo: EstadoTabuleiro,
    gerenciador_turnos: GerenciadorTurnos,
    ia: IASimples,
    processando_turno_ia: bool,
    tempo_espera_ia: f64,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for ControladorBatalha {
    fn init(base: Base<Node2D>) -> Self {
        // Inicialização HEAD (antigo)
        let jogador_humano = Jogador::novo_humano();
        let jogador_ia = JogadorIA::novo_facil();
        let fase_posicionamento = FasePosicionamento::nova();
        let fase = FaseJogo::PosicionandoJogador;
        let tempo_restante_ia = 0.0;
        let tooltip_instrucao = None;

        // Inicialização branch novo
        let tabuleiro_jogador = EstadoTabuleiro::vazio();
        let mut tabuleiro_inimigo = EstadoTabuleiro::vazio();
        let _ = tabuleiro_inimigo.posicionar_navio(2, 2); // Mock IA
        let gerenciador_turnos = GerenciadorTurnos::novo(1);
        let ia = IASimples::nova();
        let processando_turno_ia = false;
        let tempo_espera_ia = 0.0;

        Self {
            jogador_humano,
            jogador_ia,
            /// Função mesclada: une lógica de tooltip, rotação, fases, cursor e turno IA
            fn process(&mut self, delta: f64) {
                // Tooltip e rotação do fluxo antigo
                self.atualizar_tooltip_posicionamento();
                if self.fase == FaseJogo::PosicionandoJogador {
                    let input = Input::singleton();
                    if input.is_action_just_pressed("rotacionar_navio") {
                        self.fase_posicionamento.alternar_orientacao();
                        godot_print!(
                            "Orientação alterada para {}.",
                            self.fase_posicionamento.orientacao_texto().to_lowercase()
                        );
                    }
                }

                // Controle de cursor do fluxo novo
                let input = Input::singleton();
                let mouse_pos = self.base().get_global_mouse_position();
                if let Some(campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
                    cursor::controlar_cursor(campo_jogador, mouse_pos, input.clone(), "campo do jogador");
                }
                if let Some(campo_ia) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
                    cursor::controlar_cursor(campo_ia, mouse_pos, input, "campo da IA");
                }

                // Turno IA do fluxo antigo
                if self.fase == FaseJogo::TurnoIAAguardandoDelay {
                    self.tempo_restante_ia -= delta;
                    if self.tempo_restante_ia <= 0.0 {
                        self.executar_turno_ia();
                    }
                }

        if self.fase == FaseJogo::PosicionandoJogador {
            self.atualizar_preview_posicionamento();
            let input = Input::singleton();
                if self.gerenciador_turnos.estado_atual() == EstadoTurno::TurnoIA {
                    if !self.processando_turno_ia {
                        self.processando_turno_ia = true;
                        self.tempo_espera_ia = 0.0;
                    } else {
                        self.tempo_espera_ia += delta;
                        if self.tempo_espera_ia >= 1.0 {
                            self.processar_ataque_ia();
                            self.processando_turno_ia = false;
                            if !self.gerenciador_turnos.jogo_terminou() {
                                godot_print!("📋 {}", self.gerenciador_turnos.mensagem_estado());
                            }
                        }
                    }
                }
            }
            if input.is_action_just_pressed("rotacionar_navio") {
                self.fase_posicionamento.alternar_orientacao();
                godot_print!(
                    "Orientação alterada para {}.",
                    self.fase_posicionamento.orientacao_texto().to_lowercase()
                );
            }
        } else {
            self.limpar_preview_posicionamento();
        }

        if self.fase == FaseJogo::TurnoIAAguardandoDelay {
            self.tempo_restante_ia -= delta;
            if self.tempo_restante_ia <= 0.0 {
                self.executar_turno_ia();
            }
        }
    }

    /// Fluxo novo (parallax, coordenadas, etc)
    fn process_novo(&mut self, delta: f64) {
        let input = Input::singleton();
        let mouse_pos = self.base().get_global_mouse_position();
        // Controlar cursor no campo do jogador
        if let Some(campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            cursor::controlar_cursor(campo_jogador, mouse_pos, input.clone(), "campo do jogador");
        }
        // Controlar cursor no campo da IA
        if let Some(campo_ia) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
            cursor::controlar_cursor(campo_ia, mouse_pos, input, "campo da IA");
        }
        // Processar turno da IA
        if self.gerenciador_turnos.estado_atual() == EstadoTurno::TurnoIA {
            if !self.processando_turno_ia {
                self.processando_turno_ia = true;
                self.tempo_espera_ia = 0.0;
            } else {
                self.tempo_espera_ia += delta;
                if self.tempo_espera_ia >= 1.0 {
                    self.processar_ataque_ia();
                    self.processando_turno_ia = false;
                    if !self.gerenciador_turnos.jogo_terminou() {
                        godot_print!("📋 {}", self.gerenciador_turnos.mensagem_estado());
                    }
                }
            }
        }
        // Gerar coordenadas para o campo do jogador
        if let Some(campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            godot_print!("✅ CampoJogador encontrado, gerando coordenadas...");
            coordenadas::gerar_coordenadas(campo_jogador);
        }

        // Gerar coordenadas para o campo da IA
        if let Some(campo_ia) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
            godot_print!("✅ CampoIA encontrado, gerando coordenadas...");
            coordenadas::gerar_coordenadas(campo_ia);
        }

        // Mock: Pula direto para o jogo iniciado (sem fase de posicionamento)
        // TODO: Implementar fase de posicionamento de navios
        self.gerenciador_turnos.finalizar_posicionamento_jogador();
        self.ia.posicionar_navios();
        self.gerenciador_turnos.iniciar_jogo();
        
        godot_print!("📋 {}", self.gerenciador_turnos.mensagem_estado());
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mouse_pos = self.base().get_global_mouse_position();
        
        // Controlar cursor no campo do jogador
        if let Some(campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            cursor::controlar_cursor(campo_jogador, mouse_pos, input.clone(), "campo do jogador");
        }

        // Controlar cursor no campo da IA
        if let Some(campo_ia) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") {
            cursor::controlar_cursor(campo_ia, mouse_pos, input, "campo da IA");
        }

        // Processar turno da IA
        if self.gerenciador_turnos.estado_atual() == EstadoTurno::TurnoIA {
            if !self.processando_turno_ia {
                // Inicia o processamento do turno da IA
                self.processando_turno_ia = true;
                self.tempo_espera_ia = 0.0;
            } else {
                // Adiciona um pequeno delay para tornar a jogada da IA mais visível
                self.tempo_espera_ia += delta;
                if self.tempo_espera_ia >= 1.0 {  // 1 segundo de espera
                    self.processar_ataque_ia();
                    self.processando_turno_ia = false;
                    // Exibe mensagem de status se o jogo não terminou
                    if !self.gerenciador_turnos.jogo_terminou() {
                        godot_print!("📋 {}", self.gerenciador_turnos.mensagem_estado());
                    }
                }
            }
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.fase == FaseJogo::FimDeJogo {
            return;
        }
        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if !mouse_event.is_pressed() || mouse_event.get_button_index() != MouseButton::LEFT {
                return;
            }
            let click_pos = mouse_event.get_global_position();
            if self.fase == FaseJogo::PosicionandoJogador {
                self.tratar_clique_posicionamento(click_pos);
                return;
            }
            if self.fase == FaseJogo::TurnoJogador {
                self.tratar_clique_disparo_jogador(click_pos);
            }
        }
            if self.fase == FaseJogo::TurnoJogador {
                self.tratar_clique_disparo_jogador(click_pos);
            }
        }
    }
}

impl ControladorBatalha {
    fn criar_tooltip_instrucao(&mut self) {
        let mut tooltip = Label::new_alloc();
        tooltip.set_visible(false);
        tooltip.set_scale(Vector2::new(0.5, 0.5));
        self.base_mut().add_child(&tooltip.clone().upcast::<Node>());
        self.tooltip_instrucao = Some(tooltip);
    }

    fn atualizar_tooltip_posicionamento(&mut self) {
        let Some(mut tooltip) = self.tooltip_instrucao.clone() else {
            return;
        };

        if self.fase != FaseJogo::PosicionandoJogador {
            tooltip.set_visible(false);
            return;
        }

        let Some((nome, tamanho)) = self.fase_posicionamento.navio_atual() else {
            tooltip.set_visible(false);
            return;
        };

        let texto = format!(
            "Posicione: {} ({})\nClique: posicionar | R: rotacionar ({})",
            nome,
            tamanho,
            self.fase_posicionamento.orientacao_texto()
        );

        tooltip.set_text(&texto);
        tooltip.set_visible(true);

        let mouse_pos_global = self.base().get_global_mouse_position();
        let mouse_pos_local = self.base().to_local(mouse_pos_global);
        tooltip.set_position(mouse_pos_local + Vector2::new(14.0, 14.0));
    }

    fn tratar_clique_posicionamento(&mut self, click_pos: Vector2) {
        let Some(player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") else {
            return;
        };

        let Some((x, y, _)) = Self::coordenada_tabuleiro_do_clique(player_map, click_pos) else {
            return;
        };

        let nome_navio = match self.fase_posicionamento.navio_atual() {
            Some((nome, _)) => nome.to_string(),
            None => "Navio".to_string(),
        };

        match self
            .fase_posicionamento
            .tentar_posicionar_navio(&mut self.jogador_humano, x, y)
        {
            Ok(concluiu) => {
                self.atualizar_visual_meu_campo();
                if concluiu {
                    self.iniciar_fase_batalha();
                }
            }
            Err(erro) => {
                godot_print!("Não foi possível posicionar {}: {}", nome_navio, erro);
            }
        }
    }

    fn iniciar_fase_batalha(&mut self) {
        self.jogador_ia
            .jogador_mut()
            .tabuleiro_mut()
            .preencher_aleatoriamente();
        self.limpar_preview_posicionamento();
        self.fase = FaseJogo::TurnoJogador;
        godot_print!("Frotas prontas. Batalha iniciada! O jogador começa atirando.");
    }

    fn atualizar_preview_posicionamento(&mut self) {
        let Some(player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") else {
            return;
        };
        let Some(mut preview_map) = self
            .base()
            .try_get_node_as::<TileMapLayer>("PreviewPosicionamento")
        else {
            return;
        };

        let mouse_pos = self.base().get_global_mouse_position();
        let Some((x, y, _)) = Self::coordenada_tabuleiro_do_clique(player_map, mouse_pos) else {
            limpar_preview(&mut preview_map);
            return;
        };

        let Some(preview) = self
            .fase_posicionamento
            .preview_na_posicao(&self.jogador_humano, x, y)
        else {
            limpar_preview(&mut preview_map);
            return;
        };
        let Some((nome_navio, _)) = self.fase_posicionamento.navio_atual() else {
            limpar_preview(&mut preview_map);
            return;
        };
        render_preview_posicionamento(&mut preview_map, nome_navio, &preview.celulas, preview.valido);
    }

    fn limpar_preview_posicionamento(&mut self) {
        if let Some(mut preview_map) = self
            .base()
            .try_get_node_as::<TileMapLayer>("PreviewPosicionamento")
        {
            limpar_preview(&mut preview_map);
        }
    }

    fn tratar_clique_disparo_jogador(&mut self, click_pos: Vector2) {
        let Some(mut enemy_map) = self.base().try_get_node_as::<TileMapLayer>("CampoIA") else {
            return;
        };

        let Some((x, y, map_coord)) =
            Self::coordenada_tabuleiro_do_clique(enemy_map.clone(), click_pos)
        else {
            return;
        };

        let retorno = self.jogador_ia.receber_disparo(x, y);
        godot_print!("{}", retorno.mensagem);

        render_resultado_disparo(&mut enemy_map, map_coord, &retorno.resultado);

        if self.verificar_fim_de_jogo() {
            return;
        }

        if Self::disparo_foi_valido(&retorno.resultado) {
            self.fase = FaseJogo::TurnoIAAguardandoDelay;
            self.tempo_restante_ia = DELAY_TURNO_IA;
        }
    }

    fn executar_turno_ia(&mut self) {
        let Some((x, y)) = self
            .jogador_ia
            .escolher_alvo(self.jogador_humano.tabuleiro())
        else {
            self.fase = FaseJogo::FimDeJogo;
            godot_print!("Sem alvos restantes para a IA.");
            return;
        };

        let retorno = self.jogador_humano.receber_disparo(x, y);
        godot_print!("Turno da IA: {}", retorno.mensagem);

        if let Some(mut player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            render_resultado_disparo(
                &mut player_map,
                Vector2i::new(x as i32, y as i32),
                &retorno.resultado,
            );
        }

        if self.verificar_fim_de_jogo() {
            return;
        }

        self.fase = FaseJogo::TurnoJogador;
    }

    fn verificar_fim_de_jogo(&mut self) -> bool {
        if self.jogador_ia.perdeu() {
            self.fase = FaseJogo::FimDeJogo;
            godot_print!("Fim de jogo! O jogador venceu.");
            return true;
        }

        if self.jogador_humano.perdeu() {
            self.fase = FaseJogo::FimDeJogo;
            godot_print!("Fim de jogo! A IA venceu.");
            return true;
        }

        false
    }

    fn disparo_foi_valido(resultado: &ResultadoDisparo) -> bool {
        matches!(
            resultado,
            ResultadoDisparo::Agua | ResultadoDisparo::Acerto | ResultadoDisparo::Afundou(_)
        )
    }

    fn coordenada_tabuleiro_do_clique(
        map: Gd<TileMapLayer>,
        click_pos: Vector2,
    ) -> Option<(usize, usize, Vector2i)> {
        let local_pos = map.to_local(click_pos);
        let map_coord = map.local_to_map(local_pos);

        if map_coord.x < 0
            || map_coord.y < 0
            || map_coord.x >= BOARD_SIZE as i32
            || map_coord.y >= BOARD_SIZE as i32
        {
            return None;
        }

        Some((map_coord.x as usize, map_coord.y as usize, map_coord))
    }

    fn atualizar_visual_meu_campo(&mut self) {
        if let Some(mut player_map) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
            render_tabuleiro_jogador(&mut player_map, self.jogador_humano.tabuleiro());
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    let map_coord = Vector2i::new(x as i32, y as i32);
                    if let Some(celula) = self.jogador_humano.tabuleiro().valor_celula(x, y) {
                        match celula {
                            Celula::Ocupado(_) => {
                                player_map
                                    .set_cell_ex(map_coord)
                                    .source_id(0)
                                    .atlas_coords(Vector2i::new(8, 7))
                                    .done();
                            }
                            Celula::Agua => {
                                player_map
                                    .set_cell_ex(map_coord)
                                    .source_id(0)
                                    .atlas_coords(Vector2i::new(8, 3))
                                    .done();
                            }
                            Celula::Atingido(_) => {
                                player_map
                                    .set_cell_ex(map_coord)
                                    .source_id(0)
                                    .atlas_coords(Vector2i::new(10, 3))
                                    .done();
                            }
                            Celula::Vazio => {}
                        if retorno_disparo.resultado != ResultadoDisparo::JaDisparado
                            && retorno_disparo.resultado != ResultadoDisparo::ForaDosLimites
                        {
                            godot_print!("{}", retorno_disparo.mensagem);

                            match retorno_disparo.resultado {
                                ResultadoDisparo::Agua => {
                                    enemy_map
                                        .set_cell_ex(map_coord)
                                        .source_id(0)
                                        .atlas_coords(Vector2i::new(8, 3))
                                        .done();
                                    
                                    // Mock: considera que não afundou navio
                                    self.gerenciador_turnos.processar_ataque_jogador(false, false);
                                }
                                ResultadoDisparo::Acerto => {
                                    enemy_map
                                        .set_cell_ex(map_coord)
                                        .source_id(0)
                                        .atlas_coords(Vector2i::new(10, 3))
                                        .done();
                                    
                                    // Mock: considera que acerto = navio afundado (por enquanto só tem 1 navio)
                                    self.gerenciador_turnos.processar_ataque_jogador(true, true);
                                }
                                _ => {}
                            }

                            // Exibe mensagem de status se o jogo não terminou
                            if !self.gerenciador_turnos.jogo_terminou() {
                                godot_print!("📋 {}", self.gerenciador_turnos.mensagem_estado());
                            }
                        } else {
                            godot_print!("{}", retorno_disparo.mensagem);
                        }
                    }
                }
            }
        }
    }
}

#[godot_api]
impl ControladorBatalha {
    /// Processa o ataque da IA
    fn processar_ataque_ia(&mut self) {
        if let Some((x, y)) = self.ia.escolher_ataque(BOARD_SIZE as i32) {
            let retorno_disparo = executar_disparo(&mut self.tabuleiro_jogador, x as usize, y as usize);
            
            godot_print!("{}", retorno_disparo.mensagem);

            // Atualiza o campo do jogador visualmente
            if let Some(mut campo_jogador) = self.base().try_get_node_as::<TileMapLayer>("CampoJogador") {
                let map_coord = Vector2i::new(x, y);
                
                match retorno_disparo.resultado {
                    ResultadoDisparo::Agua => {
                        campo_jogador
                            .set_cell_ex(map_coord)
                            .source_id(0)
                            .atlas_coords(Vector2i::new(8, 3))
                            .done();
                        
                        // Mock: considera que não afundou navio
                        self.gerenciador_turnos.processar_ataque_ia(false, false);
                    }
                    ResultadoDisparo::Acerto => {
                        campo_jogador
                            .set_cell_ex(map_coord)
                            .source_id(0)
                            .atlas_coords(Vector2i::new(10, 3))
                            .done();
                        
                        // Mock: considera que acerto = navio afundado
                        self.gerenciador_turnos.processar_ataque_ia(true, true);
                    }
                    _ => {}
                }
            }
        }
    }
}
