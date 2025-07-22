# Component Extraction Plan - Dioxus Conversion

## Button Components

### ExploreButton
**Location**: `src/components/buttons/explore_button.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct ExploreButtonProps {
    text: String,
    href: String,
}
```
**Tailwind Classes**:
```
inline-block px-14 py-5 border border-white text-white hover:text-black
transition-all duration-300 ease-in-out relative overflow-hidden
tracking-wider uppercase text-sm
before:content-[''] before:absolute before:top-1/2 before:left-1/2 
before:w-0 before:h-0 before:bg-white before:transition-all before:duration-300
before:-translate-x-1/2 before:-translate-y-1/2 before:rounded-full
hover:before:w-[300px] hover:before:h-[300px]
hover:shadow-[0_0_30px_rgba(255,255,255,0.5)]
```
**Key Features**:
- Expanding white circle background on hover
- Text wrapped in relative z-10 span
- Uppercase text with letter spacing

### FilterButton
**Location**: `src/components/buttons/filter_button.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct FilterButtonProps {
    text: String,
    filter: String,
    active: bool,
    on_click: EventHandler<String>,
}
```
**Tailwind Classes**:
```
px-8 py-3 border border-white/30 bg-black/50 text-white cursor-pointer
transition-all duration-300 ease-in-out relative overflow-hidden
tracking-wider uppercase text-xs backdrop-blur-sm
hover:text-black hover:border-white hover:shadow-[0_0_20px_rgba(255,255,255,0.3)]
active:text-black active:border-white
```
**Key Features**:
- Active state styling
- Backdrop blur effect
- Click handler for filter changes

### ProjectLink
**Location**: `src/components/buttons/project_link.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct ProjectLinkProps {
    text: String,
    href: String,
}
```
**Tailwind Classes**:
```
text-white no-underline px-6 py-2.5 border border-white/30
transition-all duration-300 ease-in-out text-xs uppercase
tracking-wider relative overflow-hidden
hover:text-black hover:border-white
before:content-[''] before:absolute before:bottom-0 before:left-0
before:w-full before:h-0 before:bg-white before:transition-all before:duration-300
hover:before:h-full
```
**Key Features**:
- Bottom-to-top fill animation on hover
- Text wrapped in relative z-10 span

## Card Components

### ProjectCard
**Location**: `src/components/cards/project_card.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct ProjectCardProps {
    project: Project,
}

#[derive(Clone, PartialEq)]
pub struct Project {
    title: String,
    description: String,
    tags: Vec<String>,
    icon: String,
    links: HashMap<String, String>,
    number: usize,
}
```
**Tailwind Classes**:
```
border border-white/20 p-10 transition-all duration-400 ease-in-out
cursor-pointer relative overflow-hidden bg-black/50 backdrop-blur-sm
hover:translate-y-[-10px] hover:shadow-[0_20px_40px_rgba(255,255,255,0.1)]
hover:border-white/50
opacity-0 translate-y-5 animate-fadeInUp
```
**Key Features**:
- Project number overlay (absolute positioning)
- Icon with pulse animation
- Tags with hover effects
- Multiple project links
- Radial gradient glow on hover

### ContactCard
**Location**: `src/components/cards/contact_card.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct ContactCardProps {
    icon: String,
    label: String,
    href: String,
}
```
**Tailwind Classes**:
```
p-10 border border-white/30 transition-all duration-400 ease-in-out
text-white relative overflow-hidden bg-black/50 backdrop-blur-sm
hover:translate-y-[-10px] hover:shadow-[0_20px_40px_rgba(255,255,255,0.2)]
hover:border-white
```
**Key Features**:
- Large icon with transition
- Expanding circular background on hover
- Icon and label color change on hover

## Text Components

### GlitchTitle
**Location**: `src/ui/text/glitch_title.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct GlitchTitleProps {
    text: String,
}
```
**Implementation Notes**:
- Use CSS-in-JS or inline styles for complex glitch animations
- Create pseudo-elements using absolute positioned divs
- Implement clip-path animations with keyframes
**Base Classes**:
```
text-5xl sm:text-7xl lg:text-8xl font-thin tracking-[1rem] mb-4
relative text-shadow-[0_0_20px_rgba(255,255,255,0.5)]
```

### SectionTitle
**Location**: `src/ui/text/section_title.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct SectionTitleProps {
    text: String,
}
```
**Tailwind Classes**:
```
text-4xl sm:text-5xl lg:text-6xl font-thin tracking-[0.5rem] mb-8
relative inline-block
before:content-[''] before:absolute before:h-px before:w-24 before:bg-white
before:top-1/2 before:right-[calc(100%+30px)]
after:content-[''] after:absolute after:h-px after:w-24 after:bg-white
after:top-1/2 after:left-[calc(100%+30px)]
```

## Layout Components

### Tag
**Location**: `src/ui/layout/tag.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    text: String,
}
```
**Tailwind Classes**:
```
px-4 py-1.5 border border-white/30 text-xs opacity-60
uppercase tracking-wider transition-all duration-300 ease-in-out
hover:opacity-100 hover:border-white
```

## Decorative Components

### Nebula
**Location**: `src/components/decorative/nebula.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct NebulaProps {
    #[props(default = "600px")]
    width: String,
    #[props(default = "600px")]
    height: String,
    #[props(default = "20s")]
    animation_duration: String,
}
```
**Tailwind Classes**:
```
absolute blur-[100px] animate-float
bg-gradient-radial from-white/5 to-transparent
```
**Custom Animation**:
```css
@keyframes float {
    0%, 100% { transform: translate(-50%, -50%) scale(1); }
    50% { transform: translate(-30%, -70%) scale(1.2); }
}
```

### Orbit
**Location**: `src/components/decorative/orbit.rs`
**Props**:
```rust
#[derive(Props, Clone, PartialEq)]
pub struct OrbitProps {
    size: String,
    duration: String,
    reverse: bool,
}
```
**Tailwind Classes**:
```
absolute border border-white/10 rounded-full animate-spin
```
**Planet Child**:
```
absolute w-5 h-5 bg-white rounded-full -top-2.5 left-1/2
-translate-x-1/2 shadow-[0_0_20px_rgba(255,255,255,0.8)]
```

## Shared Utilities

### Animation Classes (tailwind.config.js)
```javascript
module.exports = {
  theme: {
    extend: {
      animation: {
        'float': 'float 20s ease-in-out infinite',
        'glitch': 'glitch 3s infinite',
        'glitch-1': 'glitch-1 0.5s infinite',
        'glitch-2': 'glitch-2 0.5s infinite',
        'fadeInUp': 'fadeInUp 0.8s ease forwards',
        'pulse': 'pulse 2s infinite',
      },
      keyframes: {
        float: {
          '0%, 100%': { transform: 'translate(-50%, -50%) scale(1)' },
          '50%': { transform: 'translate(-30%, -70%) scale(1.2)' }
        },
        fadeInUp: {
          'from': { opacity: '0', transform: 'translateY(30px)' },
          'to': { opacity: '1', transform: 'translateY(0)' }
        },
        pulse: {
          '0%, 100%': { transform: 'scale(1)', opacity: '0.8' },
          '50%': { transform: 'scale(1.1)', opacity: '1' }
        }
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
      }
    }
  }
}
```

## Component Patterns

### Hover Effects Pattern
Most components use a similar hover pattern:
1. Base state with subtle borders/opacity
2. Hover state with:
   - Transform (translate/scale)
   - Shadow glow effect
   - Color inversions (white text to black)
   - Background fills

### Animation Strategy
1. Use Tailwind's built-in transitions for simple effects
2. Custom keyframes for complex animations (glitch, float)
3. Stagger animations using dynamic delays
4. Consider performance with transform/opacity only

### State Management
- Filter state managed at parent level
- Active states passed as props
- Event handlers for user interactions
- No local component state needed for these UI elements