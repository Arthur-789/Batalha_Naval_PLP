use godot::classes::{Label, Node, TileMapLayer};
use godot::global::{HorizontalAlignment, VerticalAlignment};
use godot::prelude::*;

use crate::domain::tabuleiro::BOARD_SIZE;

/// Gera as coordenadas visuais (números e letras) ao redor do tabuleiro
pub fn gerar_coordenadas(mut tilemap: Gd<TileMapLayer>) {
    let tamanho_tile = 16.0;

    // Números no topo (1 a 10)
    for i in 0..BOARD_SIZE {
        let mut label = Label::new_alloc();
        let texto = format!("{}", i + 1);
        label.set_text(&texto);
        
        label.set_custom_minimum_size(Vector2::new(tamanho_tile, tamanho_tile));
        label.set_horizontal_alignment(HorizontalAlignment::CENTER);
        label.set_vertical_alignment(VerticalAlignment::CENTER);
        
        // Configurar cores
        label.add_theme_color_override("font_color", Color::from_rgb(1.0, 1.0, 1.0));
        label.add_theme_color_override("font_outline_color", Color::from_rgb(0.0, 0.0, 0.0));
        label.add_theme_constant_override("outline_size", 2);
        label.add_theme_font_size_override("font_size", 12);
        
        label.set_position(Vector2::new((i as f32) * tamanho_tile, -tamanho_tile));
        tilemap.add_child(&label.upcast::<Node>());
    }

    // Letras à esquerda (A a J)
    let letras = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    for (i, letra) in letras.iter().enumerate() {
        let mut label = Label::new_alloc();
        label.set_text(*letra);
        
        label.set_custom_minimum_size(Vector2::new(tamanho_tile, tamanho_tile));
        label.set_horizontal_alignment(HorizontalAlignment::CENTER);
        label.set_vertical_alignment(VerticalAlignment::CENTER);
        
        // Configurar cores
        label.add_theme_color_override("font_color", Color::from_rgb(1.0, 1.0, 1.0));
        label.add_theme_color_override("font_outline_color", Color::from_rgb(0.0, 0.0, 0.0));
        label.add_theme_constant_override("outline_size", 2);
        label.add_theme_font_size_override("font_size", 12);
        
        label.set_position(Vector2::new(-tamanho_tile, (i as f32) * tamanho_tile));
        tilemap.add_child(&label.upcast::<Node>());
    }

    godot_print!("✅ Coordenadas criadas para o tabuleiro");
}
