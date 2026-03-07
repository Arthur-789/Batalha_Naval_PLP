use godot::classes::{Input, Sprite2D, TileMapLayer};
use godot::prelude::*;

use crate::domain::tabuleiro::BOARD_SIZE;

/// Controla o cursor em um campo específico do tabuleiro
/// Retorna a posição do clique se ocorreu um clique válido
pub fn controlar_cursor(
    campo: Gd<TileMapLayer>,
    mouse_pos: Vector2,
    input: Gd<Input>,
    nome_campo: &str,
) -> Option<(i32, i32)> {
    let Some(mut cursor) = campo.try_get_node_as::<Sprite2D>("Sprite2D") else {
        return None;
    };

    let local_pos = campo.to_local(mouse_pos);
    let map_pos = campo.local_to_map(local_pos);

    if map_pos.x >= 0
        && map_pos.x < BOARD_SIZE as i32
        && map_pos.y >= 0
        && map_pos.y < BOARD_SIZE as i32
    {
        cursor.set_visible(true);
        
        let tamanho_tile = 16.0;
        let pos_x = (map_pos.x as f32) * tamanho_tile + (tamanho_tile / 2.0);
        let pos_y = (map_pos.y as f32) * tamanho_tile + (tamanho_tile / 2.0);
        
        cursor.set_position(Vector2::new(pos_x, pos_y));
        
        if input.is_action_just_pressed("clique_esquerdo") {
            godot_print!("🎯 Clique no {}: [{}, {}]", nome_campo, map_pos.x, map_pos.y);
            return Some((map_pos.x, map_pos.y));
        }
    } else {
        cursor.set_visible(false);
    }

    None
}
