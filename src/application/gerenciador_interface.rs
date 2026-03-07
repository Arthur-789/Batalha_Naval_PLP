use godot::classes::{FontFile, Label, Node2D, ResourceLoader};
use godot::global::HorizontalAlignment;
use godot::prelude::*;

use crate::application::gerenciador_turnos::EstadoTurno;

pub struct GerenciadorInterface {
    label_fase: Option<Gd<Label>>,
    label_turno: Option<Gd<Label>>,
    label_resultado: Option<Gd<Label>>,
}

impl GerenciadorInterface {
    pub fn novo() -> Self {
        Self {
            label_fase: None,
            label_turno: None,
            label_resultado: None,
        }
    }

    pub fn inicializar(&mut self, node: Gd<Node2D>) {
        let mut resource_loader = ResourceLoader::singleton();
        let font_path = "res://fonts/Retro Gaming.ttf";
        
        let font = resource_loader
            .load(font_path)
            .and_then(|res| res.try_cast::<FontFile>().ok());

        if let Some(mut label_fase) = node.try_get_node_as::<Label>("LabelFase") {
            if let Some(font_file) = font.clone() {
                label_fase.add_theme_font_override("font", &font_file);
            }
            label_fase.add_theme_font_size_override("font_size", 24);
            label_fase.add_theme_color_override("font_color", Color::from_rgb(0.0, 0.0, 0.0));
            label_fase.add_theme_color_override("font_outline_color", Color::from_rgb(1.0, 1.0, 1.0));
            label_fase.add_theme_constant_override("outline_size", 3);
            label_fase.set_horizontal_alignment(HorizontalAlignment::CENTER);
            label_fase.set_position(Vector2::new(80.0, 20.0));
            label_fase.set_size(Vector2::new(400.0, 50.0));
            label_fase.set_z_index(100);
            self.label_fase = Some(label_fase);
        }

        if let Some(mut label_turno) = node.try_get_node_as::<Label>("LabelTurno") {
            if let Some(font_file) = font.clone() {
                label_turno.add_theme_font_override("font", &font_file);
            }
            label_turno.add_theme_font_size_override("font_size", 18);
            label_turno.add_theme_color_override("font_color", Color::from_rgb(0.0, 0.0, 0.0));
            label_turno.add_theme_color_override("font_outline_color", Color::from_rgb(1.0, 1.0, 1.0));
            label_turno.add_theme_constant_override("outline_size", 3);
            label_turno.set_horizontal_alignment(HorizontalAlignment::CENTER);
            label_turno.set_position(Vector2::new(80.0, 70.0));
            label_turno.set_size(Vector2::new(400.0, 40.0));
            label_turno.set_z_index(100);
            self.label_turno = Some(label_turno);
        }

        if let Some(mut label_resultado) = node.try_get_node_as::<Label>("LabelResultado") {
            if let Some(font_file) = font {
                label_resultado.add_theme_font_override("font", &font_file);
            }
            label_resultado.add_theme_font_size_override("font_size", 32);
            label_resultado.add_theme_color_override("font_color", Color::from_rgb(0.0, 0.0, 0.0));
            label_resultado.add_theme_color_override("font_outline_color", Color::from_rgb(1.0, 1.0, 1.0));
            label_resultado.add_theme_constant_override("outline_size", 4);
            label_resultado.set_horizontal_alignment(HorizontalAlignment::CENTER);
            label_resultado.set_position(Vector2::new(80.0, 300.0));
            label_resultado.set_size(Vector2::new(400.0, 60.0));
            label_resultado.set_z_index(100);
            label_resultado.set_visible(false);
            self.label_resultado = Some(label_resultado);
        }
    }

    pub fn atualizar(&mut self, estado: EstadoTurno, rodada: u32) {
        match estado {
            EstadoTurno::SelecaoDificuldade => {
                if let Some(mut label_fase) = self.label_fase.clone() {
                    label_fase.set_text("Selecione a Dificuldade");
                    label_fase.set_visible(true);
                }
                if let Some(mut label_turno) = self.label_turno.clone() {
                    label_turno.set_visible(false);
                }
                if let Some(mut label_resultado) = self.label_resultado.clone() {
                    label_resultado.set_visible(false);
                }
            }
            EstadoTurno::PosicionamentoJogador => {
                if let Some(mut label_fase) = self.label_fase.clone() {
                    label_fase.set_text("Fase de Posicionamento");
                    label_fase.set_visible(true);
                }
                if let Some(mut label_turno) = self.label_turno.clone() {
                    label_turno.set_visible(false);
                }
                if let Some(mut label_resultado) = self.label_resultado.clone() {
                    label_resultado.set_visible(false);
                }
            }
            EstadoTurno::TurnoJogador | EstadoTurno::TurnoIA => {
                if let Some(mut label_fase) = self.label_fase.clone() {
                    label_fase.set_text(&format!("Rodada {}", rodada));
                    label_fase.set_visible(true);
                }
                if let Some(mut label_turno) = self.label_turno.clone() {
                    if estado == EstadoTurno::TurnoJogador {
                        label_turno.set_text("Sua vez!");
                    } else {
                        label_turno.set_text("Turno da IA");
                    }
                    label_turno.set_visible(true);
                }
                if let Some(mut label_resultado) = self.label_resultado.clone() {
                    label_resultado.set_visible(false);
                }
            }
            EstadoTurno::VitoriaJogador => {
                if let Some(mut label_fase) = self.label_fase.clone() {
                    label_fase.set_visible(false);
                }
                if let Some(mut label_turno) = self.label_turno.clone() {
                    label_turno.set_visible(false);
                }
                if let Some(mut label_resultado) = self.label_resultado.clone() {
                    label_resultado.set_text("Vitoria!");
                    label_resultado.set_visible(true);
                }
            }
            EstadoTurno::VitoriaIA => {
                if let Some(mut label_fase) = self.label_fase.clone() {
                    label_fase.set_visible(false);
                }
                if let Some(mut label_turno) = self.label_turno.clone() {
                    label_turno.set_visible(false);
                }
                if let Some(mut label_resultado) = self.label_resultado.clone() {
                    label_resultado.set_text("Derrota!");
                    label_resultado.set_visible(true);
                }
            }
            EstadoTurno::PosicionamentoIA => {
                // Estado transitório, não precisa mostrar nada
            }
        }
    }
}
