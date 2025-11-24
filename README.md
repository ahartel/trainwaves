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

## Understanding the train system

In this section, I write down my understanding of the train system used in this game.
My understanding derives from reading the code of the simple-godot-train project, its documentation and from asking Claude about it.

One thing I learned when reading the Track code is that it is what godot calls a [tool](https://docs.godotengine.org/en/stable/tutorials/plugins/running_code_in_the_editor.html). Tools in godot are scripts that run in the editor as well as at runtime.

> @tool is a powerful line of code that, when added at the top of your script, makes it execute in the editor. You can also decide which parts of the script execute in the editor, which in game, and which in both.

This means that the Track nodes in the scene tree are actually running code while you are editing the scene in the godot editor. This allows the Track nodes to be rendered as tracks, with crossties, even while editing a scene.

### System Architecture Overview
The train following system is built on 5 layers that work together:
1. Infrastructure Layer (The Rails)
 - Track.gd - Defines curved paths using Godot's Path2D system. Each track segment knows its entry/exit points and can link to other tracks
 - TrackSwitch.gd - Y-junctions that route trains left or right based on switch position
 - TrackJunction.gd - Auto-connects tracks when they touch, building the track network
2. Movement Core Layer (The Wheels)
 - Bogie.gd - This is where track following actually happens! A bogie is a set of wheels that uses Godot's PathFollow2D to move along track curves by adjusting its progress property (distance along the curve in pixels)
3. Vehicle Layer (The Train Cars)
 - TrainVehicle.gd - A train car with two bogies (front and back). The car body positions itself between the bogies and rotates to face the direction of travel
4. Physics Layer (The Power)
 - TrainEngine.gd - Extends TrainVehicle to add realistic physics: calculates forces (throttle, friction, drag, brakes), updates velocity using F=ma, and moves the front bogie
5. Input Layer (The Controls)
 - player_control_component.gd - Captures keyboard input and controls the engine's throttle and brakes

### How a Train Follows Tracks: The Flow
#### Step 1: Player Input → Physics
```
Keyboard input → player_control_component 
→ Sets TrainEngine.target_force_percent
→ Engine calculates net force (power - friction - drag - brakes)
→ Updates velocity using physics (F = ma)
```
#### Step 2: Physics → Movement
```
TrainEngine calls front_bogie.move(velocity * delta)
→ Bogie adjusts its progress along the track curve
→ progress += distance (or -= if going backward)
→ PathFollow2D automatically positions and rotates the bogie
```
#### Step 3: Track Transitions
```When bogie reaches track end (progress < 0 or > track_length):
→ Bogie emits at_track_head or at_track_tail signal
→ Current track forwards signal to connected track
→ Next track calls bogie.set_track() (reparents bogie)
→ Bogie continues on new track with proper direction
```
#### Step 4: Multi-Bogie Propagation (The Clever Part!)
```
Front bogie emits "moved" signal after each movement
→ Back bogie receives signal via move_as_follower()
→ Back bogie positions itself follow_distance behind front
→ If another car is coupled, its front bogie follows the back bogie
→ Chain reaction through entire train consist
```
#### Step 5: Visual Smoothing
```
TrainVehicle body lerps toward front_bogie position
→ Body rotates to look_at back_bogie
→ Creates smooth, natural train appearance
```

### Key Algorithms
#### Track Following (The Core)
The system uses Godot's ` PathFollow2D.progress` property:
 - `progress` = pixel distance along the baked curve
- Godot automatically handles positioning and rotation along the Bezier curve
- Moving is just: `progress += distance`

#### Leader-Follower (Keeping Cars Together)
Hybrid approach:
- Mid-track: Snap follower to exact distance behind leader (rigid coupling)
- Near boundaries: Use incremental movement to handle track transitions smoothly

#### Track Network (Routing)
- Tracks form a signal-based graph structure
- Each track links to neighbors via `link_track()`
- TrackSwitch adds conditional routing based on switch position
- TrackJunction auto-detects and creates connections

#### Physics Simulation
- `net_force = applied_force - friction - rolling_resistance - air_drag - brakes`
- `velocity += (net_force / total_mass) * delta`

### The Magic Moment
When a train moves, here's what happens every frame:
1. Engine physics calculates how much force to apply based on throttle, mass, and resistance
2. Front bogie moves along its track curve by updating its progress
3. Curve following happens automatically via PathFollow2D - the bogie "sticks" to the rail
4. Track transition triggers if the bogie reaches the end, seamlessly moving to the next track
5. Back bogie follows at a fixed distance, triggering its own track transitions if needed
6. Additional cars follow in a chain reaction through the coupled signal system
7. Visual bodies smoothly interpolate to create natural-looking train movement

This signal-based, physics-driven approach creates realistic train behavior where cars naturally follow each other around curves, through switches, and across complex track networks!