extends Area2D

@onready var timer: Timer = $Timer
var current_train = null

func _on_area_entered(area: Area2D) -> void:
	timer.start()
	area.get_parent().brake_force = 0.0
	current_train = area
	print("Train arrived.")


func _on_timer_timeout() -> void:
	current_train.get_parent().velocity = 2.0
	current_train.get_parent().target_force_percent = 0.4
