extends GDPlayer

@onready var input_manager: GDInputManager = $GDInputManager
@onready var replicated_node: GDReplicatedNode = $".."

func _physics_process(_delta: float) -> void:
	var direction = Input.get_vector(
		"move_left", 
		"move_right", 
		"move_down", 
		"move_up"
	)
	
	if is_locally_owned():
		input_manager.send_input(replicated_node.net_id, direction)


func _on_boat_deserialize(bytes: Array[int]) -> void:
	deserialize_bytes(bytes)
	pass
