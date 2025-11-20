# trainwaves

A train ~~tower~~ station defense game.
Trains come in waves and need to be steered into their desired stations.

In this game, you play as a train station manager.
You start with a fixed level layout with tracks and switches and signals.
The level represents a larger train station.

Upon start, the game first generates a train schedule.
In the first version of the game, this will probably be a hard-coded schedule.

Based on that schedule the game will then generate people who are waiting for these trains.
In the first version, we will only represent people as simple colored squares.
They will be present from the beginning and will be patiently waiting for their train to arrive.
They only care about the color of their train, where color also represents the destination of the train.
They don't mind waiting until their train arrives but they expect it to arrive and depart on time.
In fact, it's even OK for the train to arrive early as long as it departs on time.

When the trains start arriving, the player needs to switch tracks and signals so that each train arrives on time at its designated platform.
If a train arrives late, the people waiting for that train will get upset which causes the player to lose points.
If a train arrives on time, the people will happily board the train and the player gains points.

If a train arrives at the wrong platform, people will need time to switch to the correct platform.
This causes delays and makes the people unhappy, causing the player to lose points.

In later versions of the game, there might be more complex behavior for the people.
In later versions of the game, there might be some unexpected events happening, like train delays, track blockages, broken signals and switches.

Harder levels have more complex track layouts, more trains, more people and tighter schedules.

The game is for the Godot game engine and relies heavily on the great project
[simple-godot-train](https://github.com/moonbench/simple-godot-train) by
[moonbench](https://github.com/moonbench/simple-godot-train).
The files in Assets, Scenes and Scripts are Copyright (c) 2022 Moonbench under MIT license.