use crate::domain::entidades::usuario::Usuario;
use crate::domain::entidades::conquista::Conquista;

pub struct ConquistaService;
    
impl ConquistaService {

    pub fn adicionar_conquista(&self, usuario: &mut Usuario, conquista: Conquista) {
        usuario.adicionar_conquista(conquista);
    }

    pub fn listar_conquistas<'a>(&self, usuario: &'a Usuario) -> &'a Vec<Conquista> {
        &usuario.conquistas
    }
}
    