extends Node2D

@export var car_count = 4
@export var train_vehicle_reference : PackedScene

@onready var engine : TrainEngine = $TrainEngine

func _setup_train():
	engine.add_to_track($Tracks/Track)
	engine.velocity = 2.0
	engine.target_force_percent = 0.4
	
	var last_car = engine
	for index in range(car_count):
		var car = train_vehicle_reference.instantiate()
		add_child(car)
		last_car.set_follower_car(car)
		last_car = car

func _ready() -> void:
	_setup_train()
