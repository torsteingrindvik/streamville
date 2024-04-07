# Ideas

### Short/medium projects

## grid visualizer 3D

- Start a 3D visualizer thing
- Imagine a 3D grid of say W x H x Z size
- Each grid point has
    - First: A sphere
    - Later: Billboard textured something
- Each grid point has a measure of good/bad
    - Sphere: Green vs red

### Why?

Let's say you are doing some profiling. E.g. a compute shader.

The workgroup indices represent our grid dimensions.

We can show the results of profiling visually in 3D using this approach.

#### Sphere

Make sphere green or red based on best/worst performance.

#### Texture

If we screenshot shader using different parameters (up to three), we can display the resulting
shader given those params.

## Entity spawn/despawn visualizer

Possibly with timeline scrubber.
Green is spawn, red is despawn.

## Bathroom breakdance

Flying loo

## BRB thing

### Option 1

BRB text flying into/out of screen in happy colors, as a Bevy app (no window)

### Option 2

Foxes spelling out BRB by their position where position is decoded from a font

## Book of shaders

Each part in isolation can be short

## Depth of Field

There was this nice article we should find that

## Portals

- Place orange portal
- Place blue portal

Both point in the direction of the normal of attached wall.
Choose some aspect ratio of portal, e.g. 3:10 (something tall-ish).

Find a suitable resolution to render that at.

Render after main.

Display on wall.

Wobbly effects! Rounded effect! Flamelike effect!

## Twitch profile pic

"Must be JPEG, PNG, or GIF and cannot exceed 10MB."

We should definitely generate this somehow using a shader.

## Twitch profile banner

"recommended 1200x480, max 10MB" 

Generate! Shader!

## Foids

It's boids but foxes

## ~~Spiral fox bullet hell~~

_Done: See `bullet-hell.rs`_

Typical bullet-hell outwards spiral thing, but
each projectile samples Bevy logo where UV is mapped from world pos

## Bevdown / Markevy

Markdown files rendered in Bevy

- Header sizes respected
- Treesitter syntax highlight?

### Longer projects

## Ghost of Tsushima grass

From that nice video.

- Compute shader produces mesh is cool
- That thing where they smoothly transition the mesh into lower detail by distance
- Wind via shader noise

## That thing Townscaper does

I forget but specifically meshes plopping into existence
in a nice wobbly way 

## Stream is You

Figure out stream API then have a list of would-be followers, subs.
Map these to entities somehow, then display


