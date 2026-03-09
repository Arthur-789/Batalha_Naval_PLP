extends Node2D

const CAMPANHA_SCENE_PATH := "res://scenes/modo_campanha.tscn"
const MENU_SCENE_PATH := "res://MenuPrincipal.tscn"
const VITORIA_SCENE_PATH := "res://scenes/tela_vitoria.tscn"
const DERROTA_SCENE_PATH := "res://scenes/tela_derrota.tscn"

@onready var controlador: Node = $ControladorBatalha

func _ready() -> void:
	if not CampaignState.em_campanha:
		return

	if controlador.has_signal("batalha_encerrada"):
		controlador.connect("batalha_encerrada", Callable(self, "_on_batalha_encerrada"))

	_call_forced_campaign_difficulty()

func _call_forced_campaign_difficulty() -> void:
	match CampaignState.vitorias:
		0:
			controlador.call("selecionar_dificuldade_facil")
		1:
			controlador.call("selecionar_dificuldade_media")
		2:
			controlador.call("selecionar_dificuldade_dificil")
		_:
			controlador.call("selecionar_dificuldade_dificil")

func _on_batalha_encerrada(vitoria: bool) -> void:
	if not CampaignState.em_campanha:
		return

	var usuario_node = UsuarioNode.new()
	var login_atual = ""

	var caminho_absoluto = ProjectSettings.globalize_path("res://dados/usuario_atual.json")
	var file = FileAccess.open(caminho_absoluto, FileAccess.READ)

	if file:
		var json_text = file.get_as_text()
		file.close()
		print("DEBUG_ARQUIVO_LIDO: ", json_text)

		var dict = JSON.parse_string(json_text)
		if dict is Dictionary:
			login_atual = dict.get("login", "")
	else:
		print("DEBUG_ERRO: Não conseguiu abrir -> ", caminho_absoluto)

	print("DEBUG_LOGIN_FINAL: ", login_atual)

	if vitoria:
		CampaignState.registrar_vitoria()
		if login_atual != "":
			var sucesso = usuario_node.registrar_vitoria(login_atual)
			print("DEBUG_VITORIA_SALVA: ", sucesso)

		if CampaignState.vitorias >= 3 or CampaignState.campanha_concluida:
			get_tree().change_scene_to_file(VITORIA_SCENE_PATH)
		else:
			get_tree().change_scene_to_file(CAMPANHA_SCENE_PATH)
		return

	CampaignState.registrar_derrota()
	if login_atual != "":
		var sucesso = usuario_node.registrar_derrota(login_atual)
		print("DEBUG_DERROTA_SALVA: ", sucesso)

	get_tree().change_scene_to_file(DERROTA_SCENE_PATH)
