use std::collections::HashMap;
use crate::types::Project;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Flutter UI Designs".to_string(),
            description: "Collection of responsive UI designs with parallax animations. 309+ stars on GitHub.".to_string(),
            tags: vec!["flutter".to_string(), "mobile".to_string(), "web".to_string(), "open-source".to_string()],
            icon: "◐".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/flutter-ui-designs".to_string());
                links.insert("demo".to_string(), "#".to_string());
                links
            },
            number: 1,
        },
        Project {
            title: "React Native Components".to_string(),
            description: "Cross-platform mobile components with native performance and smooth animations.".to_string(),
            tags: vec!["react-native".to_string(), "mobile".to_string(), "open-source".to_string()],
            icon: "◑".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "#".to_string());
                links.insert("npm".to_string(), "#".to_string());
                links
            },
            number: 2,
        },
        Project {
            title: "Rust Web Framework".to_string(),
            description: "High-performance web framework built with Rust, featuring async capabilities and zero-copy deserialization.".to_string(),
            tags: vec!["rust".to_string(), "web".to_string(), "open-source".to_string()],
            icon: "◒".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "#".to_string());
                links.insert("docs".to_string(), "#".to_string());
                links
            },
            number: 3,
        },
        Project {
            title: "Neural Network Visualizer".to_string(),
            description: "Interactive web application for visualizing neural network architectures and training processes.".to_string(),
            tags: vec!["web".to_string(), "react-native".to_string(), "mobile".to_string()],
            icon: "◓".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("demo".to_string(), "#".to_string());
                links.insert("github".to_string(), "#".to_string());
                links
            },
            number: 4,
        },
        Project {
            title: "Blockchain Explorer".to_string(),
            description: "Real-time blockchain transaction explorer with advanced filtering and analytics capabilities.".to_string(),
            tags: vec!["web".to_string(), "rust".to_string(), "open-source".to_string()],
            icon: "◔".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("live".to_string(), "#".to_string());
                links.insert("github".to_string(), "#".to_string());
                links
            },
            number: 5,
        },
        Project {
            title: "Mobile Game Engine".to_string(),
            description: "Lightweight 2D game engine optimized for mobile devices with built-in physics and particle systems.".to_string(),
            tags: vec!["flutter".to_string(), "mobile".to_string(), "open-source".to_string()],
            icon: "◕".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("docs".to_string(), "#".to_string());
                links.insert("github".to_string(), "#".to_string());
                links
            },
            number: 6,
        },
    ]
}