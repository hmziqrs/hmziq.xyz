use std::collections::HashMap;
use crate::types::Project;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Flutter UI Designs".to_string(),
            description: "Collection of UI designs built with Flutter, runs on any platform: mobile, web & desktop. 320+ stars on GitHub.".to_string(),
            tags: vec!["flutter".to_string(), "dart".to_string(), "mobile".to_string(), "web".to_string(), "desktop".to_string()],
            icon: "◐".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/flutter-ui-designs".to_string());
                links
            },
            number: 1,
        },
        Project {
            title: "InvMovieConcept1".to_string(),
            description: "True Cross platform UI design featuring complex scroll based animations powered by Flutter. 61 stars on GitHub.".to_string(),
            tags: vec!["flutter".to_string(), "dart".to_string(), "ui".to_string(), "animation".to_string()],
            icon: "◑".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/invmovieconcept1".to_string());
                links
            },
            number: 2,
        },
        Project {
            title: "React Native Loop Game".to_string(),
            description: "Clone of infinity loop game built with React Native. 22 stars on GitHub.".to_string(),
            tags: vec!["react-native".to_string(), "typescript".to_string(), "game".to_string(), "mobile".to_string()],
            icon: "◒".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/react-native-loop-game".to_string());
                links
            },
            number: 3,
        },
        Project {
            title: "Flutter Experiments".to_string(),
            description: "Collection of experimental Flutter projects and UI concepts. 15 stars on GitHub.".to_string(),
            tags: vec!["flutter".to_string(), "dart".to_string(), "experimental".to_string()],
            icon: "◓".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/Flutter-Experiments".to_string());
                links
            },
            number: 4,
        },
        Project {
            title: "Ruxlog Frontend".to_string(),
            description: "Modern frontend application built with TypeScript for the Ruxlog platform.".to_string(),
            tags: vec!["typescript".to_string(), "web".to_string(), "frontend".to_string()],
            icon: "◔".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/ruxlog-frontend".to_string());
                links
            },
            number: 5,
        },
        Project {
            title: "Go Cross Screenshot".to_string(),
            description: "Small Go program to take screenshots across Windows, Linux & macOS platforms.".to_string(),
            tags: vec!["go".to_string(), "cross-platform".to_string(), "utility".to_string()],
            icon: "◕".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/go-cross-screenshot".to_string());
                links
            },
            number: 6,
        },
        Project {
            title: "FhglLaunch".to_string(),
            description: "Flutter application launcher with custom features and animations.".to_string(),
            tags: vec!["flutter".to_string(), "dart".to_string(), "launcher".to_string()],
            icon: "◖".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/fhgllaunch".to_string());
                links
            },
            number: 7,
        },
        Project {
            title: "Personal CV".to_string(),
            description: "Personal CV built with Dioxus SSG - a Rust-based static site generator.".to_string(),
            tags: vec!["rust".to_string(), "dioxus".to_string(), "ssg".to_string(), "web".to_string()],
            icon: "◗".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/cv".to_string());
                links
            },
            number: 8,
        },
        Project {
            title: "Ruxlog Backend".to_string(),
            description: "Rust-based backend service for the Ruxlog platform with modern async architecture.".to_string(),
            tags: vec!["rust".to_string(), "backend".to_string(), "async".to_string()],
            icon: "◐".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/ruxlog-backend".to_string());
                links
            },
            number: 9,
        },
        Project {
            title: "Gandalf Sax".to_string(),
            description: "Behold the glory of epic infinite gandalf - a fun Flutter project.".to_string(),
            tags: vec!["flutter".to_string(), "dart".to_string(), "fun".to_string()],
            icon: "◑".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/gandalf-sax".to_string());
                links
            },
            number: 10,
        },
        Project {
            title: "Next.js Blog".to_string(),
            description: "Modern blog built with Next.js featuring SSR and React components.".to_string(),
            tags: vec!["nextjs".to_string(), "react".to_string(), "typescript".to_string(), "ssr".to_string()],
            icon: "◒".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/next-blog".to_string());
                links
            },
            number: 11,
        },
        Project {
            title: "Dioxus Free Icons".to_string(),
            description: "Use free SVG icons in your Dioxus projects easily - a Rust icon library.".to_string(),
            tags: vec!["rust".to_string(), "dioxus".to_string(), "icons".to_string(), "library".to_string()],
            icon: "◓".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/dioxus-free-icons".to_string());
                links
            },
            number: 12,
        },
    ]
}