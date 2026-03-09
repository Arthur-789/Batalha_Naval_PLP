use godot::prelude::*;
use godot::classes::{Button, Control, IControl, Label, PackedScene, ResourceLoader, TextureRect, VBoxContainer};
use std::fs;

pub struct RegistroRanking {
    pub nome_login: String,
    pub pontuacao: i32,
    pub is_current_user: bool,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct CenaRanking {
    base: Base<Control>,
}

#[godot_api]
impl IControl for CenaRanking {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let usuario_atual = self.obter_usuario_atual();
        let mut dados = self.ler_dados_ranking(&usuario_atual);

        dados.sort_by(|a, b| b.pontuacao.cmp(&a.pontuacao));

        self.popular_lista(dados);

        let mut btn_voltar = self.base().get_node_as::<Button>("botao_voltar");
        let callable = self.base().callable("voltar_menu");
        btn_voltar.connect("pressed", &callable);
    }
}

#[godot_api]
impl CenaRanking {
    fn obter_usuario_atual(&self) -> String {
        if let Ok(conteudo) = fs::read_to_string("usuario_atual.json") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&conteudo) {
                if let Some(login) = json.get("login").and_then(|v| v.as_str()) {
                    return login.to_string();
                }
            }
        }
        String::new()
    }

    fn ler_dados_ranking(&self, usuario_atual: &str) -> Vec<RegistroRanking> {
        let mut lista_jogadores = Vec::new();

        let mut json_string = String::new();
        if let Ok(conteudo) = fs::read_to_string("dados/usuarios.json") {
            json_string = conteudo;
        } else if let Ok(conteudo) = fs::read_to_string("usuarios.json") {
            json_string = conteudo;
        }

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_string) {
            if let Some(array) = json.as_array() {
                for item in array {
                    let login = item.get("login").and_then(|v| v.as_str()).unwrap_or("Desconhecido").to_string();
                    let vitorias = item.get("vitorias").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let pontuacao = vitorias * 100;

                    lista_jogadores.push(RegistroRanking {
                        is_current_user: login == usuario_atual,
                        nome_login: login,
                        pontuacao,
                    });
                }
            }
        }

        lista_jogadores
    }

    fn popular_lista(&mut self, dados: Vec<RegistroRanking>) {
        let mut lista_container = self.base().get_node_as::<VBoxContainer>("VBoxContainer");

        let mut resource_loader = ResourceLoader::singleton();
        let cena_linha = resource_loader
            .load("res://scenes/linha_jogador.tscn")
            .unwrap()
            .cast::<PackedScene>();

        for (index, jogador) in dados.into_iter().enumerate() {
            let mut nova_linha = cena_linha.instantiate().unwrap();

            let mut label_nome = nova_linha.get_node_as::<Label>("nick");
            label_nome.set_text(&jogador.nome_login);

            let mut label_pontuacao = nova_linha.get_node_as::<Label>("pontuacao");
            let texto_pontuacao = format!("{}", jogador.pontuacao);
            label_pontuacao.set_text(&texto_pontuacao);

            if jogador.is_current_user {
                label_nome.set_modulate(Color::from_rgb(1.0, 0.84, 0.0));
                label_pontuacao.set_modulate(Color::from_rgb(1.0, 0.84, 0.0));
            }

            let mut icone_trofeu = nova_linha.get_node_as::<TextureRect>("trofeu");
            if index == 0 {
                icone_trofeu.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 1.0));
            } else {
                icone_trofeu.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 0.0));
            }

            lista_container.add_child(&nova_linha);
        }
    }

    #[func]
    fn voltar_menu(&mut self) {
        let mut tree = self.base().get_tree();
        tree.change_scene_to_file("res://MenuPrincipal.tscn");
    }
}