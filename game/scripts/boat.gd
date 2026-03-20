extends GDPlayer

var speed :float = 300.0
		
func _physics_process(delta: float) -> void:
	var direction = Input.get_vector("move_left", "move_right", "move_down", "move_up")
	
	velocity = direction * speed * delta;
	
	move_and_slide()


func _on_boat_deserialize(bytes: Array[int]) -> void:
	deserialize(bytes)
