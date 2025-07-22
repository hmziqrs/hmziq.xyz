# Project Structure

```
hmziq.xyz/
├── assets/              # Static assets (favicon, CSS, SVG files)
├── src/
│   ├── main.rs         # Entry point with App component
│   ├── router.rs       # Route definitions (Route enum)
│   ├── config.rs       # Configuration settings
│   ├── env.rs          # Environment variables
│   ├── components/     # Reusable UI components
│   ├── containers/     # Container components (logic + UI)
│   ├── screens/        # Page-level components
│   │   ├── home/       # Home screen module
│   │   ├── not_found.rs # 404 page
│   │   └── mod.rs
│   ├── hooks/          # Custom hooks
│   │   ├── use_form.rs # Form management hook
│   │   ├── use_previous.rs # Previous value tracking
│   │   └── mod.rs
│   ├── store/          # State management
│   │   ├── lib.rs      # StateFrame and ApiError
│   │   └── mod.rs
│   └── ui/             # UI component library
│       └── mod.rs
├── Cargo.toml          # Rust dependencies
├── Dioxus.toml         # Dioxus configuration
├── tailwind.css        # Tailwind input file
├── guide.md            # Development guide
└── README.md           # Project documentation
```

## Key Files
- **main.rs**: Application entry point with asset loading and Router setup
- **router.rs**: Route enum definition for navigation
- **hooks/**: Custom hooks including form management and state tracking
- **store/lib.rs**: StateFrame pattern for async operation state management