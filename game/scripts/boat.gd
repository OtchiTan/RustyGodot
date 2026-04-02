extends GDPlayer

@onready var input_manager: GDInputManager = $"../GDInputManager"

func _process(_delta: float) -> void:
	var direction = Input.get_vector(
		"move_left", 
		"move_right", 
		"move_down", 
		"move_up"
	)
	
	if is_locally_owned():
		input_manager.add_direction_input(direction)

func _on_boat_deserialize(stream_reader: GDStreamReader) -> void:
	deserialize_bytes(stream_reader)
