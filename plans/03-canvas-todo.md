# Canvas Implementation TODO - Dioxus Conversion

## Overview

The canvas element in the original HTML provides an interactive space background with:
- Dynamic star field with depth effect
- Star trails for fast-moving stars
- Constellation system with animated connections
- Mouse interaction that connects nearby stars
- Performance-optimized rendering loop

## Canvas Components to Implement

### 1. SpaceCanvas Component

**Location**: `src/components/canvas/space_canvas.rs`

```rust
use dioxus::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsCast;

#[component]
pub fn SpaceCanvas() -> Element {
    let canvas_ref = use_node_ref();
    
    use_effect(move || {
        if let Some(canvas) = canvas_ref.get() {
            // Initialize canvas and start animation loop
        }
    });
    
    rsx! {
        canvas {
            id: "space-canvas",
            class: "fixed inset-0 -z-10",
            node_ref: canvas_ref,
        }
    }
}
```

### 2. Star System

```rust
#[derive(Clone)]
struct Star {
    x: f64,
    y: f64,
    z: f64,
    size: f64,
    speed: f64,
}

impl Star {
    fn new(width: f64, height: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        Self {
            x: rng.gen_range(0.0..width),
            y: rng.gen_range(0.0..height),
            z: rng.gen_range(0.0..1000.0),
            size: rng.gen_range(0.0..2.0),
            speed: rng.gen_range(0.01..0.06),
        }
    }
    
    fn update(&mut self, width: f64, height: f64) {
        self.z -= self.speed * 10.0;
        if self.z <= 0.0 {
            self.x = rand::thread_rng().gen_range(0.0..width);
            self.y = rand::thread_rng().gen_range(0.0..height);
            self.z = 1000.0;
        }
    }
    
    fn draw(&self, ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
        let x = (self.x - width / 2.0) * (1000.0 / self.z) + width / 2.0;
        let y = (self.y - height / 2.0) * (1000.0 / self.z) + height / 2.0;
        let s = self.size * (1000.0 / self.z);
        let opacity = 1.0 - self.z / 1000.0;
        
        // Draw star
        ctx.begin_path();
        ctx.set_fill_style(&format!("rgba(255, 255, 255, {})", opacity).into());
        ctx.arc(x, y, s, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.fill();
        
        // Draw trail for fast-moving stars
        if self.z < 300.0 {
            ctx.begin_path();
            ctx.set_stroke_style(&format!("rgba(255, 255, 255, {})", opacity * 0.5).into());
            ctx.set_line_width(s * 0.5);
            ctx.move_to(x, y);
            
            let prev_x = (self.x - width / 2.0) * (1000.0 / (self.z + self.speed * 20.0)) + width / 2.0;
            let prev_y = (self.y - height / 2.0) * (1000.0 / (self.z + self.speed * 20.0)) + height / 2.0;
            
            ctx.line_to(prev_x, prev_y);
            ctx.stroke();
        }
    }
}
```

### 3. Constellation System

```rust
#[derive(Clone)]
struct ConstellationPoint {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[derive(Clone)]
struct Constellation {
    points: Vec<ConstellationPoint>,
    connections: Vec<(usize, usize)>,
}

impl Constellation {
    fn new(width: f64, height: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let num_points = rng.gen_range(3..7);
        
        let center_x = rng.gen_range(0.0..width);
        let center_y = rng.gen_range(0.0..height);
        
        let mut points = Vec::new();
        for _ in 0..num_points {
            points.push(ConstellationPoint {
                x: center_x + (rng.gen_range(-100.0..100.0)),
                y: center_y + (rng.gen_range(-100.0..100.0)),
                vx: rng.gen_range(-0.05..0.05),
                vy: rng.gen_range(-0.05..0.05),
            });
        }
        
        // Create connections
        let mut connections = Vec::new();
        for i in 0..num_points - 1 {
            connections.push((i, i + 1));
        }
        if rng.gen_bool(0.5) {
            connections.push((num_points - 1, 0));
        }
        
        Self { points, connections }
    }
    
    fn update(&mut self, width: f64, height: f64) {
        for point in &mut self.points {
            point.x += point.vx;
            point.y += point.vy;
            
            if point.x < -100.0 || point.x > width + 100.0 {
                point.vx *= -1.0;
            }
            if point.y < -100.0 || point.y > height + 100.0 {
                point.vy *= -1.0;
            }
        }
    }
    
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        // Draw connections
        ctx.set_stroke_style(&"rgba(255, 255, 255, 0.1)".into());
        ctx.set_line_width(1.0);
        
        for (i, j) in &self.connections {
            ctx.begin_path();
            ctx.move_to(self.points[*i].x, self.points[*i].y);
            ctx.line_to(self.points[*j].x, self.points[*j].y);
            ctx.stroke();
        }
        
        // Draw points
        ctx.set_fill_style(&"rgba(255, 255, 255, 0.5)".into());
        for point in &self.points {
            ctx.begin_path();
            ctx.arc(point.x, point.y, 2.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
            ctx.fill();
        }
    }
}
```

### 4. Mouse Interaction System

```rust
fn draw_mouse_connections(
    ctx: &CanvasRenderingContext2d,
    stars: &[Star],
    mouse_x: f64,
    mouse_y: f64,
    width: f64,
    height: f64,
) {
    for star in stars {
        let x = (star.x - width / 2.0) * (1000.0 / star.z) + width / 2.0;
        let y = (star.y - height / 2.0) * (1000.0 / star.z) + height / 2.0;
        let dist = ((x - mouse_x).powi(2) + (y - mouse_y).powi(2)).sqrt();
        
        if dist < 100.0 && star.z < 500.0 {
            let opacity = (1.0 - dist / 100.0) * (1.0 - star.z / 500.0);
            
            ctx.begin_path();
            ctx.set_stroke_style(&format!("rgba(255, 255, 255, {})", opacity * 0.3).into());
            ctx.set_line_width(1.0);
            ctx.move_to(mouse_x, mouse_y);
            ctx.line_to(x, y);
            ctx.stroke();
        }
    }
}
```

## Implementation Strategy

### Phase 1: WASM Canvas Setup
1. Install dependencies:
   ```toml
   [dependencies]
   web-sys = { version = "0.3", features = [
       "CanvasRenderingContext2d",
       "HtmlCanvasElement",
       "Window",
       "Document",
   ]}
   wasm-bindgen = "0.2"
   rand = { version = "0.8", features = ["wasm-bindgen"] }
   ```

2. Create canvas initialization hook
3. Set up resize handler
4. Create basic render loop

### Phase 2: Star System
1. Implement Star struct and methods
2. Create star pool (500+ stars)
3. Add update and render logic
4. Implement star trails for depth

### Phase 3: Constellation System
1. Implement Constellation struct
2. Add random generation logic
3. Create drift animation
4. Render connections and points

### Phase 4: Mouse Interactions
1. Track mouse position globally
2. Calculate distances to stars
3. Draw connections to nearby stars
4. Add glow effect at cursor position

### Phase 5: Performance Optimization
1. Use requestAnimationFrame for smooth animation
2. Implement object pooling for stars
3. Use canvas layers if needed
4. Profile and optimize render calls

## Alternative CSS-Only Approach

If canvas performance is an issue or for initial implementation:

### 1. CSS Stars
```rust
#[component]
fn CSSStarField() -> Element {
    rsx! {
        div { class: "fixed inset-0 -z-10 overflow-hidden",
            for i in 0..200 {
                div {
                    class: "absolute w-1 h-1 bg-white rounded-full animate-twinkle",
                    style: format!(
                        "left: {}%; top: {}%; animation-delay: {}s; animation-duration: {}s",
                        rand::thread_rng().gen_range(0..100),
                        rand::thread_rng().gen_range(0..100),
                        rand::thread_rng().gen_range(0.0..5.0),
                        rand::thread_rng().gen_range(3.0..8.0)
                    )
                }
            }
        }
    }
}
```

### 2. CSS Constellations
```css
@keyframes constellation-drift {
    0% { transform: translate(0, 0); }
    50% { transform: translate(30px, -20px); }
    100% { transform: translate(0, 0); }
}

.constellation {
    position: absolute;
    animation: constellation-drift 30s ease-in-out infinite;
}

.constellation-line {
    position: absolute;
    height: 1px;
    background: rgba(255, 255, 255, 0.1);
    transform-origin: left center;
}
```

### 3. Glow Cursor (Already CSS)
The cursor glow is already implemented as a CSS element, so it can be kept as is.

## Performance Considerations

### Canvas Approach
- Pro: Smooth animations, full control, better for complex effects
- Con: Higher CPU usage, requires WASM setup
- Best for: Desktop experiences

### CSS Approach
- Pro: GPU accelerated, simpler implementation
- Con: Limited interaction possibilities, more DOM elements
- Best for: Mobile/lightweight experiences

## Recommended Implementation Order

1. Start with CSS-only version for quick results
2. Implement basic canvas without interactions
3. Add star system with depth effect
4. Add constellation system
5. Implement mouse interactions
6. Optimize performance
7. Add toggle for users to switch between CSS/Canvas

## Testing Strategy

1. Test on different screen sizes
2. Check performance on mobile devices
3. Verify WASM loading works correctly
4. Test with different star counts
5. Profile render performance
6. Check memory usage over time

## Future Enhancements

1. Add shooting stars/comets
2. Implement nebula clouds with Perlin noise
3. Add particle effects on click
4. Create warp speed effect on navigation
5. Add planet orbits with realistic physics
6. Implement day/night cycle based on time