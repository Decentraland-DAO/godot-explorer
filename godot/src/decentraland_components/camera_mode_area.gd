extends DclCameraModeArea3D

@onready var collision_shape_3d = $CollisionShape3D


# Called when the node enters the scene tree for the first time.
func _ready():
	var shape = BoxShape3D.new()
	shape.set_size(area)
	collision_shape_3d.set_shape(shape)
	collision_shape_3d.set_disabled(true)


func _on_scene_active(active: bool):
	collision_shape_3d.set_disabled(!active)
