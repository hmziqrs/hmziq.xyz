# Ather.js - Particle Flow Animation

A canvas-based particle system that creates organic, flowing visual effects.

## Core Components

### Particle System
- 2000 particles with 9 properties each: `[x, y, vx, vy, speed, hue, width, life, ttl]`
- Spawns randomly in 150-300px radius from center
- Each particle has random hue (160-260), speed (1-8), width (0.1-2), and lifetime (50-200 frames)

### Movement Algorithm
- Uses SimplexNoise3D to generate organic flow patterns
- Velocity lerped towards noise-generated direction (5% blend per frame)
- Creates natural, flowing movement paths

### Rendering Pipeline
1. **Buffer Canvas**: Draws particle trails
2. **Main Canvas**: Applies effects in layers:
   - Black fade overlay (10% opacity) for trail effect
   - Blurred copy with "lighten" blend mode for glow
   - Sharp copy with "lighter" blend mode for brightness

### Lifecycle
- Particles fade in/out based on life/ttl ratio
- Auto-respawn when out of bounds or expired
- Continuous animation loop via requestAnimationFrame

## Key Functions
- `initParticle(i)`: ref/ather.js:36 - Sets random initial values
- `drawParticle(i)`: ref/ather.js:59 - Updates position and renders trail
- `draw()`: ref/ather.js:107 - Main render loop with compositing

## Visual Result
Creates ethereal, flowing streams of colored light that move organically across the screen with glowing trails.