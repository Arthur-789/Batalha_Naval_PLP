use godot::classes::RandomNumberGenerator;
use godot::prelude::*;

use crate::domain::estrategias_ia::EstrategiaIA;
use crate::domain::tabuleiro::{Celula, EstadoTabuleiro, MovimentoNavio, BOARD_SIZE};

pub struct EstrategiaFacil;

impl EstrategiaIA for EstrategiaFacil {
    fn escolher_alvo(&mut self, tabuleiro_inimigo: &EstadoTabuleiro) -> Option<(usize, usize)> {
        let mut alvos_disponiveis = Vec::new();

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if let Some(celula) = tabuleiro_inimigo.valor_celula(x, y) {
                    if matches!(celula, Celula::Vazio | Celula::Ocupado(_)) {
                        alvos_disponiveis.push((x, y));
                    }
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

    fn escolher_movimento(
        &mut self,
        meu_tabuleiro: &EstadoTabuleiro,
        _tiros_inimigo: &[[bool; BOARD_SIZE]; BOARD_SIZE],
    ) -> Option<MovimentoNavio> {
        let mut rng = RandomNumberGenerator::new_gd();
        rng.randomize();

        if rng.randf() < 0.45 {
            return None;
        }

        let movimentos = meu_tabuleiro.listar_movimentos_validos();
        if movimentos.is_empty() {
            return None;
        }

        let idx = rng.randi_range(0, (movimentos.len() - 1) as i32) as usize;
        Some(movimentos[idx])
    }
}
