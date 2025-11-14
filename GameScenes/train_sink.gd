extends Node2D



func _on_area_2d_area_entered(area: Area2D) -> void:
	get_tree().call_group("train_waggons", "queue_free")
	get_tree().call_group("train_engines", "queue_free")
	get_tree().call_group("bogies", "queue_free")
