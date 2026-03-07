extends VBoxContainer


# Called when the node enters the scene tree for the first time.
func _ready():
	$MargemBotoes/botoes/start.grab_focus()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _on_start_pressed():
	print("pressionou start")
