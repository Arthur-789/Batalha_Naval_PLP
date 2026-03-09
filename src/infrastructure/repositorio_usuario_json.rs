use godot::prelude::*;
use godot::classes::FileAccess;
use godot::classes::file_access::ModeFlags;
use crate::domain::entidades::usuario::Usuario;
use crate::domain::repositorios::repositorio_usuario::RepositorioUsuario;

pub struct RepositorioUsuarioJson {
    path: String,
    users: Vec<Usuario>
}

impl RepositorioUsuarioJson {
    pub fn new(path: &str) -> Self {
        let mut data = String::from("[]");

        if FileAccess::file_exists(path) {
            data = FileAccess::get_file_as_string(path).to_string();
        }

        let users: Vec<Usuario> = serde_json::from_str(&data).unwrap_or_default();

        Self {
            path: path.to_string(),
            users
        }
    }

    fn persist(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.users)
            .map_err(|e| e.to_string())?;

        if let Some(mut file) = FileAccess::open(&self.path, ModeFlags::WRITE) {
            file.store_string(&json);
            file.flush();
            file.close();
            Ok(())
        } else {
            Err("Falha ao salvar o arquivo pelo Godot".to_string())
        }
    }
}

impl RepositorioUsuario for RepositorioUsuarioJson {
    fn salvar(&mut self, usuario: Usuario) -> Result<(), String> {
        self.users.push(usuario);
        self.persist()
    }

    fn achar_por_login(&self, login: &str) -> Option<Usuario> {
        self.users.iter().find(|u| u.login == login).cloned()
    }

    fn achar_por_id(&self, id: u64) -> Option<Usuario> {
        self.users.iter().find(|u| u.id == id).cloned()
    }

    fn atualizar(&mut self, usuario: Usuario) -> Result<(), String> {
        let pos = self.users.iter().position(|u| u.id == usuario.id)
            .ok_or("Usuário não encontrado".to_string())?;
        self.users[pos] = usuario;
        self.persist()
    }

    fn excluir(&mut self, id: u64) -> Result<(), String> {
        let pos = self.users.iter().position(|u| u.id == id)
            .ok_or("Usuário não encontrado".to_string())?;
        self.users.remove(pos);
        self.persist()
    }

    fn listar(&self) -> Vec<Usuario> {
        self.users.clone()
    }
}