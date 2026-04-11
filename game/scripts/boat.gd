extends GDPlayer

@onready var input_manager: GDInputManager = $"../GDInputManager"
@onready var sprite: AnimatedSprite2D = $AnimatedSprite2D

const ANIMATION_FRAMES = [
	"idle_r",
	"idle_tr",
	"idle_t",
	"idle_tl",
	"idle_l",
	"idle_bl",
	"idle_b",
	"idle_br",
]

func _process(_delta: float) -> void:
	if is_locally_owned():
		var direction = Input.get_vector(
			"move_left",
			"move_right",
			"move_down",
			"move_up"
		)
		
		if direction != Vector2.ZERO:
			var orientation = roundi(rad_to_deg(direction.angle()));
			if orientation < 0:
				orientation = 360 + orientation
			sprite.play(ANIMATION_FRAMES[clampi(orientation / 45.0, 0, 7)])
			
		input_manager.add_direction_input(direction)



func _on_boat_deserialize(snap1: Array[int], snap2: Array[int], alpha: float) -> void:
	call_deferred("deserialize_bytes", snap1, snap2, alpha)
