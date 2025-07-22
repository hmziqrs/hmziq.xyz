# Page Assembly Plan - Dioxus Conversion

## Screen Structure

```
src/screens/home/
├── mod.rs          # Main HomeScreen component
├── hero.rs         # Hero section with glitch title
├── projects.rs     # Projects grid with filtering
└── contact.rs      # Contact links section
```

## Implementation Steps

### 1. HomeScreen Component (`src/screens/home/mod.rs`)

```rust
use dioxus::prelude::*;
use crate::components::decorative::CursorGlow;

#[component]
pub fn HomeScreen() -> Element {
    // Scroll position for parallax
    let scroll_y = use_signal(|| 0.0);
    
    // Mouse position for cursor effects
    let mouse_pos = use_signal(|| (0.0, 0.0));
    
    // Active filter state
    let active_filter = use_signal(|| "all".to_string());
    
    rsx! {
        div {
            class: "bg-black text-white overflow-x-hidden leading-relaxed cursor-crosshair min-h-screen",
            
            // Canvas placeholder - will be implemented later
            div { id: "space-canvas", class: "fixed inset-0 -z-10" }
            
            // Cursor glow effect
            CursorGlow { position: mouse_pos() }
            
            // Sections
            HeroSection { scroll_y: scroll_y() }
            ProjectsSection { active_filter }
            ContactSection {}
            
            // Floating asteroids
            Asteroids {}
        }
    }
}
```

### 2. HeroSection Component (`src/screens/home/hero.rs`)

```rust
use dioxus::prelude::*;
use crate::components::{
    buttons::ExploreButton,
    text::GlitchTitle,
    decorative::{Nebula, Orbit}
};

#[component]
pub fn HeroSection(scroll_y: f64) -> Element {
    rsx! {
        section {
            class: "h-screen relative flex items-center justify-center overflow-hidden",
            
            // Parallax nebulas
            Nebula {
                width: "600px",
                height: "600px",
                animation_duration: "20s",
                style: format!("transform: translate(-50%, {}%)", -50.0 + scroll_y * 0.1)
            }
            Nebula {
                width: "800px",
                height: "800px",
                animation_duration: "30s",
                style: format!("transform: translate(-50%, {}%)", -50.0 + scroll_y * 0.15)
            }
            
            // Orbiting elements
            Orbit {
                size: "300px",
                duration: "60s",
                reverse: false,
                class: "top-[10%] -right-[150px]"
            }
            Orbit {
                size: "500px",
                duration: "90s",
                reverse: true,
                class: "-bottom-[250px] -left-[250px]"
            }
            
            // Hero content
            div {
                class: "text-center z-10 relative",
                
                GlitchTitle { text: "DIGITAL LABORATORY" }
                
                p {
                    class: "text-base sm:text-xl lg:text-2xl font-light tracking-[0.5rem] opacity-80 animate-fadeInUp animation-delay-300 uppercase",
                    "Experiments in Code & Design"
                }
                
                div {
                    class: "mt-16 animate-fadeInUp animation-delay-600",
                    ExploreButton {
                        text: "INITIATE EXPLORATION",
                        href: "#projects"
                    }
                }
            }
            
            // Coordinates display
            div {
                class: "absolute bottom-8 left-8 font-mono text-xs opacity-50 tracking-wider",
                span { "COORD: " }
                span { id: "coordText", "00.000° N, 00.000° W" }
            }
        }
    }
}
```

### 3. ProjectsSection Component (`src/screens/home/projects.rs`)

```rust
use dioxus::prelude::*;
use crate::components::{
    text::SectionTitle,
    buttons::FilterButton,
    cards::ProjectCard
};
use crate::store::home::get_projects;

#[component]
pub fn ProjectsSection(active_filter: Signal<String>) -> Element {
    let projects = get_projects();
    let filters = vec!["all", "mobile", "web", "rust", "flutter", "react-native", "open-source"];
    
    let filtered_projects = use_memo(move || {
        let filter = active_filter();
        if filter == "all" {
            projects.clone()
        } else {
            projects.iter()
                .filter(|p| p.tags.contains(&filter))
                .cloned()
                .collect()
        }
    });
    
    rsx! {
        section {
            id: "projects",
            class: "py-36 px-5 max-w-[1400px] mx-auto relative",
            
            // Section header
            div {
                class: "text-center mb-24 relative",
                SectionTitle { text: "EXPERIMENTS" }
            }
            
            // Filter container
            div {
                class: "flex justify-center gap-5 mb-20 flex-wrap relative",
                class: "before:content-[''] before:absolute before:top-1/2 before:left-0 before:right-0",
                class: "before:h-px before:bg-gradient-to-r before:from-transparent before:via-white/20 before:to-transparent before:-z-10",
                
                for filter in filters {
                    FilterButton {
                        text: filter.to_uppercase(),
                        filter: filter.to_string(),
                        active: active_filter() == filter,
                        on_click: move |f| active_filter.set(f)
                    }
                }
            }
            
            // Projects grid
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10",
                
                for (index, project) in filtered_projects().iter().enumerate() {
                    div {
                        style: format!("animation-delay: {}ms", index * 100),
                        ProjectCard { project: project.clone() }
                    }
                }
            }
        }
    }
}
```

### 4. ContactSection Component (`src/screens/home/contact.rs`)

```rust
use dioxus::prelude::*;
use crate::components::{
    text::SectionTitle,
    cards::ContactCard
};

#[component]
pub fn ContactSection() -> Element {
    let contacts = vec![
        ("⟁", "GITHUB", "https://github.com/hmziqrs"),
        ("◈", "TWITTER", "https://twitter.com/hmziqrs"),
        ("◉", "TELEGRAM", "https://t.me/hmziqrs"),
        ("◆", "WEBSITE", "https://www.hmziq.rs/"),
    ];
    
    rsx! {
        section {
            id: "contact",
            class: "py-36 px-5 text-center relative overflow-hidden",
            class: "before:content-[''] before:absolute before:top-0 before:left-0 before:right-0",
            class: "before:h-px before:bg-gradient-to-r before:from-transparent before:via-white before:to-transparent",
            
            div {
                class: "mb-20",
                SectionTitle { text: "ESTABLISH CONTACT" }
            }
            
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-10 max-w-5xl mx-auto",
                
                for (icon, label, href) in contacts {
                    ContactCard {
                        icon: icon.to_string(),
                        label: label.to_string(),
                        href: href.to_string()
                    }
                }
            }
        }
    }
}
```

### 5. Asteroids Component (`src/screens/home/mod.rs`)

```rust
#[component]
fn Asteroids() -> Element {
    rsx! {
        for i in 0..10 {
            div {
                class: "fixed w-[3px] h-[3px] bg-white rounded-full opacity-60 animate-asteroid",
                style: format!(
                    "top: {}%; animation-delay: {}s; animation-duration: {}s",
                    (i * 10) % 100,
                    i as f32 * 2.0,
                    15.0 + (i as f32 * 0.5)
                )
            }
        }
    }
}
```

## State Management

### Filter State (`src/store/home/mod.rs`)

```rust
pub mod projects;

pub use projects::get_projects;
```

### Projects Data (`src/store/home/projects.rs`)

```rust
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub icon: String,
    pub links: HashMap<String, String>,
    pub number: usize,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Flutter UI Designs".to_string(),
            description: "Collection of responsive UI designs with parallax animations. 309+ stars on GitHub.".to_string(),
            tags: vec!["flutter", "mobile", "web", "open-source"].into_iter().map(String::from).collect(),
            icon: "◐".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/flutter-ui-designs".to_string());
                links.insert("demo".to_string(), "#".to_string());
                links
            },
            number: 1,
        },
        // Add all other projects...
    ]
}
```

## Event Handlers

### Scroll Handler (in main.rs or HomeScreen)

```rust
use_effect(move || {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let scroll_handler = Closure::wrap(Box::new(move || {
        let scroll_y = window.page_y_offset().unwrap_or(0.0);
        scroll_y.set(scroll_y);
    }) as Box<dyn FnMut()>);
    
    window.add_event_listener_with_callback(
        "scroll",
        scroll_handler.as_ref().unchecked_ref()
    ).unwrap();
    
    move || {
        // Cleanup on unmount
    }
});
```

### Mouse Handler (similar pattern)

```rust
use_effect(move || {
    let document = web_sys::window().unwrap().document().unwrap();
    
    let mouse_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        mouse_pos.set((event.client_x() as f64, event.client_y() as f64));
        
        // Update coordinates display
        if let Some(coord_elem) = document.get_element_by_id("coordText") {
            let lat = (event.client_y() as f64 / window_height * 180.0 - 90.0);
            let lng = (event.client_x() as f64 / window_width * 360.0 - 180.0);
            let coord_text = format!(
                "{:.3}° {}, {:.3}° {}",
                lat.abs(),
                if lat > 0.0 { "N" } else { "S" },
                lng.abs(),
                if lng > 0.0 { "E" } else { "W" }
            );
            coord_elem.set_inner_html(&coord_text);
        }
    }) as Box<dyn FnMut(_)>);
    
    document.add_event_listener_with_callback(
        "mousemove",
        mouse_handler.as_ref().unchecked_ref()
    ).unwrap();
});
```

## Tailwind Configuration Updates

```javascript
// tailwind.config.js
module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      animation: {
        'float': 'float 20s ease-in-out infinite',
        'glitch': 'glitch 3s infinite',
        'glitch-1': 'glitch-1 0.5s infinite',
        'glitch-2': 'glitch-2 0.5s infinite',
        'fadeInUp': 'fadeInUp 0.8s ease forwards',
        'pulse': 'pulse 2s infinite',
        'asteroid': 'asteroid 20s linear infinite',
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
        asteroid: {
          'from': { transform: 'translateX(-100px) translateY(0) rotate(0deg)' },
          'to': { transform: 'translateX(calc(100vw + 100px)) translateY(100px) rotate(360deg)' }
        }
      },
      animationDelay: {
        '300': '300ms',
        '600': '600ms',
      }
    }
  },
  plugins: [],
}
```

## Router Integration

Update `src/router.rs`:

```rust
#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomeScreen {},
}
```

## Build Steps

1. Create all component files according to the extraction plan
2. Implement screen components in order: Hero → Projects → Contact
3. Add state management for filters and scroll
4. Configure Tailwind with custom animations
5. Test responsive behavior and animations
6. Optimize bundle size and performance

## Performance Considerations

1. Use `use_memo` for filtered projects to avoid recalculation
2. Debounce scroll and mouse events
3. Lazy load project images if added later
4. Consider virtual scrolling for large project lists
5. Minimize re-renders with proper signal usage

## Accessibility

1. Add proper ARIA labels to interactive elements
2. Ensure keyboard navigation for filters
3. Provide alt text for decorative elements
4. Test with screen readers
5. Ensure sufficient color contrast