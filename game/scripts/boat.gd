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



func _on_boat_deserialize(snap1: Array[int], snap2: Array[int], alpha: float) -> void:
	call_deferred("deserialize_bytes", snap1, snap2, alpha)
