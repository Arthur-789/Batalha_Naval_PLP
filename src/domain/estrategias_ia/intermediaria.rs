use godot::classes::RandomNumberGenerator;
use godot::prelude::*;

use crate::domain::disparo::RetornoDisparo;
use crate::domain::estrategias_ia::EstrategiaIA;
use crate::domain::tabuleiro::{Celula, EstadoTabuleiro, MovimentoNavio, BOARD_SIZE};

#[derive(Debug, Clone, Copy, PartialEq)]
enum DirecaoBusca {
    Norte,
    Sul,
    Leste,
    Oeste,
}

pub struct EstrategiaIntermediaria {
    acertos_ativos: Vec<(usize, usize)>,
    direcao_atual: Option<DirecaoBusca>,
    navios_afundados: Vec<String>,
}

impl EstrategiaIntermediaria {
    pub fn nova() -> Self {
        Self {
            acertos_ativos: Vec::new(),
            direcao_atual: None,
            navios_afundados: Vec::new(),
        }
    }

    fn obter_adjacentes(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacentes = Vec::new();
        
        if x > 0 {
            adjacentes.push((x - 1, y));
        }
        if x < BOARD_SIZE - 1 {
            adjacentes.push((x + 1, y));
        }
        if y > 0 {
            adjacentes.push((x, y - 1));
        }
        if y < BOARD_SIZE - 1 {
            adjacentes.push((x, y + 1));
        }
        
        adjacentes
    }

    fn obter_proxima_na_direcao(&self, x: usize, y: usize, direcao: DirecaoBusca) -> Option<(usize, usize)> {
        match direcao {
            DirecaoBusca::Norte if x > 0 => Some((x - 1, y)),
            DirecaoBusca::Sul if x < BOARD_SIZE - 1 => Some((x + 1, y)),
            DirecaoBusca::Oeste if y > 0 => Some((x, y - 1)),
            DirecaoBusca::Leste if y < BOARD_SIZE - 1 => Some((x, y + 1)),
            _ => None,
        }
    }

    fn determinar_direcao(&self, p1: (usize, usize), p2: (usize, usize)) -> Option<DirecaoBusca> {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        if x1 == x2 {
            if y2 > y1 {
                Some(DirecaoBusca::Leste)
            } else {
                Some(DirecaoBusca::Oeste)
            }
        } else if y1 == y2 {
            if x2 > x1 {
                Some(DirecaoBusca::Sul)
            } else {
                Some(DirecaoBusca::Norte)
            }
        } else {
            None
        }
    }

    fn celula_disponivel(&self, tabuleiro: &EstadoTabuleiro, x: usize, y: usize) -> bool {
        if let Some(celula) = tabuleiro.valor_celula(x, y) {
            matches!(celula, Celula::Vazio | Celula::Ocupado(_))
        } else {
            false
        }
    }

    fn escolher_aleatorio(&self, tabuleiro: &EstadoTabuleiro) -> Option<(usize, usize)> {
        let mut alvos_disponiveis = Vec::new();

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.celula_disponivel(tabuleiro, x, y) {
                    alvos_disponiveis.push((x, y));
                }
            }
        }

        if alvos_disponiveis.is_empty() {
            return None;
        }

        let mut rng = RandomNumberGenerator::new_gd();
        rng.randomize();
        let idx = rng.randi_range(0, (alvos_disponiveis.len() - 1) as i32) as usize;
        Some(alvos_disponiveis[idx])
    }
}

impl EstrategiaIA for EstrategiaIntermediaria {
    fn escolher_alvo(&mut self, tabuleiro_inimigo: &EstadoTabuleiro) -> Option<(usize, usize)> {
        if self.acertos_ativos.is_empty() {
            return self.escolher_aleatorio(tabuleiro_inimigo);
        }

        if self.acertos_ativos.len() >= 2 && self.direcao_atual.is_none() {
            if let Some(direcao) = self.determinar_direcao(
                self.acertos_ativos[0],
                self.acertos_ativos[1],
            ) {
                self.direcao_atual = Some(direcao);
            }
        }

        if let Some(direcao) = self.direcao_atual {
            let ultimo = *self.acertos_ativos.last().unwrap();
            if let Some(proximo) = self.obter_proxima_na_direcao(ultimo.0, ultimo.1, direcao) {
                if self.celula_disponivel(tabuleiro_inimigo, proximo.0, proximo.1) {
                    return Some(proximo);
                }
            }

            let primeiro = self.acertos_ativos[0];
            let direcao_oposta = match direcao {
                DirecaoBusca::Norte => DirecaoBusca::Sul,
                DirecaoBusca::Sul => DirecaoBusca::Norte,
                DirecaoBusca::Leste => DirecaoBusca::Oeste,
                DirecaoBusca::Oeste => DirecaoBusca::Leste,
            };
            if let Some(proximo) = self.obter_proxima_na_direcao(primeiro.0, primeiro.1, direcao_oposta) {
                if self.celula_disponivel(tabuleiro_inimigo, proximo.0, proximo.1) {
                    return Some(proximo);
                }
            }
        }

        for &(x, y) in &self.acertos_ativos {
            let adjacentes = self.obter_adjacentes(x, y);
            for (ax, ay) in adjacentes {
                if self.celula_disponivel(tabuleiro_inimigo, ax, ay) {
                    return Some((ax, ay));
                }
            }
        }

        self.escolher_aleatorio(tabuleiro_inimigo)
    }

    fn notificar_resultado(&mut self, x: usize, y: usize, resultado: &RetornoDisparo) {
        match &resultado.resultado {
            crate::domain::disparo::ResultadoDisparo::Acerto => {
                self.acertos_ativos.push((x, y));
            }
            crate::domain::disparo::ResultadoDisparo::Afundou(nome_navio) => {
                self.acertos_ativos.clear();
                self.direcao_atual = None;
                self.navios_afundados.push(nome_navio.clone());
            }
            _ => {}
        }
    }

    fn escolher_movimento(
        &mut self,
        meu_tabuleiro: &EstadoTabuleiro,
        _tiros_inimigo: &[[bool; BOARD_SIZE]; BOARD_SIZE],
    ) -> Option<MovimentoNavio> {
        let mut rng = RandomNumberGenerator::new_gd();
        rng.randomize();

        if rng.randf() < 0.30 {
            return None;
        }

        let mut movimentos_prioritarios = Vec::new();
        let movimentos = meu_tabuleiro.listar_movimentos_validos();
        if movimentos.is_empty() {
            return None;
        }

        for movimento in &movimentos {
            let Some(navio) = meu_tabuleiro.navios.get(movimento.navio_idx) else {
                continue;
            };
            if navio.acertos > 0 && !navio.esta_afundado() {
                movimentos_prioritarios.push(*movimento);
            }
        }

        let escolha = if movimentos_prioritarios.is_empty() {
            &movimentos
        } else {
            &movimentos_prioritarios
        };
        let idx = rng.randi_range(0, (escolha.len() - 1) as i32) as usize;
        Some(escolha[idx])
    }
}
