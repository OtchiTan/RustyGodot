extends GDPlayer

@onready var sprite: AnimatedSprite2D = $AnimatedSprite2D
var orientation := 0

const ANIMATION_FRAMES = [
	"r",
	"tr",
	"t",
	"tl",
	"l",
	"bl",
	"b",
	"br",
]

func _process(_delta: float) -> void:
	var state = "idle"
	
	if velocity != Vector2.ZERO:
		state = "move"
		orientation = roundi(rad_to_deg(velocity.angle()));
		if orientation < 0:
			orientation = 360 + orientation
	
	var animation = ANIMATION_FRAMES[clampf(orientation / 45.0, 0, 7)];
	sprite.play(state + "_" + animation)




func _on_boat_deserialize(snap1: Array[int], snap2: Array[int], alpha: float) -> void:
	call_deferred("deserialize_bytes", snap1, snap2, alpha)
