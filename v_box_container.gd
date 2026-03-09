extends VBoxContainer

const SESSION_FILE_PATH := "res://dados/usuario_atual.json"
const LOGIN_SCENE_PATH := "res://TelaLogin.tscn"

func _ready():
	$start.grab_focus()

	if not $start.pressed.is_connected(_on_start_pressed):
		$start.pressed.connect(_on_start_pressed)
	if not $dinamico.pressed.is_connected(_on_dinamico_pressed):
		$dinamico.pressed.connect(_on_dinamico_pressed)
	if not $ranking.pressed.is_connected(_on_ranking_pressed):
		$ranking.pressed.connect(_on_ranking_pressed)
	if not $conquistas.pressed.is_connected(_on_conquistas_pressed):
		$conquistas.pressed.connect(_on_conquistas_pressed)
	if not $sair.pressed.is_connected(_on_sair_pressed):
		$sair.pressed.connect(_on_sair_pressed)

func _on_start_pressed():
	CampaignState.iniciar_nova_campanha()
	get_tree().change_scene_to_file("res://scenes/modo_campanha.tscn")

func _on_dinamico_pressed():
	CampaignState.iniciar_nova_campanha_dinamica()
	get_tree().change_scene_to_file("res://scenes/modo_campanha.tscn")

func _on_ranking_pressed():
	get_tree().change_scene_to_file("res://scenes/tela_ranking.tscn")

func _on_conquistas_pressed():
	get_tree().change_scene_to_file("res://scenes/CenaConquistas.tscn")

func _on_sair_pressed():
	var file := FileAccess.open(SESSION_FILE_PATH, FileAccess.WRITE)
	if file != null:
		file.store_string("{}")
		file.close()
	get_tree().change_scene_to_file(LOGIN_SCENE_PATH)
