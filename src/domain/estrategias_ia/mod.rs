pub mod facil;
pub mod intermediaria;
pub mod dificil;

use crate::domain::disparo::RetornoDisparo;
use crate::domain::tabuleiro::{EstadoTabuleiro, MovimentoNavio, BOARD_SIZE};

pub trait EstrategiaIA {
    fn escolher_alvo(&mut self, tabuleiro_inimigo: &EstadoTabuleiro) -> Option<(usize, usize)>;
    fn notificar_resultado(&mut self, _x: usize, _y: usize, _resultado: &RetornoDisparo) {}
    fn configurar_modo_dinamico(&mut self, _ativo: bool) {}
    fn escolher_movimento(
        &mut self,
        _meu_tabuleiro: &EstadoTabuleiro,
        _tiros_inimigo: &[[bool; BOARD_SIZE]; BOARD_SIZE],
    ) -> Option<MovimentoNavio> {
        None
    }
}

pub use facil::EstrategiaFacil;
pub use intermediaria::EstrategiaIntermediaria;
pub use dificil::EstrategiaDificil;
