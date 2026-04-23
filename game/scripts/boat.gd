extends GDPlayer

@onready var input_manager: GDInputManager = $"../GDInputManager"
@onready var sprite: AnimatedSprite2D = $AnimatedSprite2D

@export var boat_speed = 500.0;

var orientation := 0

const ANIMATION_FRAMES = [
	"r",
	"br",
	"b",
	"bl",
	"l",
	"tl",
	"t",
	"tr",
]

func _process(_delta: float) -> void:
	if is_locally_owned():
		var direction = Input.get_vector(
			"move_left",
			"move_right",
			"move_up",
			"move_down",
		)
		
		var state = "idle"
		
		if direction != Vector2.ZERO:
			state = "move"
			orientation = roundi(rad_to_deg(direction.angle()));
			if orientation < 0:
				orientation = 360 + orientation
		
		var animation = ANIMATION_FRAMES[clampf(orientation / 45.0, 0, 7)];
		sprite.play(state + "_" + animation)
		
		velocity = direction * boat_speed
		move_and_slide()
		
		input_manager.add_direction_input(direction)
		



func _on_boat_deserialize(snap1: Array[int], snap2: Array[int], alpha: float) -> void:
	call_deferred("deserialize_bytes", snap1, snap2, alpha)
