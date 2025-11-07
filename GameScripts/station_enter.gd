extends Area2D


func _on_area_entered(area: Area2D) -> void:
	area.get_parent().target_force_percent = 0.0
	area.get_parent().brake_force = 2.0
	print("Train arriving...")
