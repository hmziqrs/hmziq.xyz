use std::collections::HashMap;
use crate::types::Project;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Flutter UI Designs".to_string(),
            short_description: "Cross-platform Flutter UI collection for mobile, web and desktop".to_string(),
            description: "Collection of UI designs built with Flutter that runs on any platform including mobile, web, and desktop. Features responsive layouts, animations, and internationalization with various UI implementations like food delivery apps, parallax designs, and concept interfaces. Includes downloadable releases for all major platforms and a live web demo.".to_string(),
            tags: vec!["desktop-app".to_string(), "dart".to_string(), "ui-design".to_string(), "cross-platform".to_string(), "parallax".to_string(), "offline-first".to_string(), "web-application".to_string(), "animations".to_string(), "flutter".to_string(), "parallax-scrolling".to_string(), "flutter-apps".to_string(), "flutter-examples".to_string(), "flutter-app".to_string(), "flutter-desktop".to_string(), "flutter-design".to_string(), "flutter-web".to_string(), "flutter-animations".to_string()],
            clean_tags: vec!["flutter".to_string(), "ui-design".to_string(), "animations".to_string()],
            stars: 320,
            forks: 54,
            icon: "◈".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/flutter-ui-designs".to_string());
                links.insert("website".to_string(), "https://flutter-uis.hmziq.xyz/".to_string());
                links.insert("playstore".to_string(), "https://play.google.com/store/apps/details?id=com.onemdev.flutter_ui_challenges".to_string());
                links
            },
            number: 1,
        },
        Project {
            title: "Inv Movie Concept 1".to_string(),
            short_description: "Cross-platform movie UI with complex scroll animations in Flutter".to_string(),
            description: "True cross-platform UI design featuring complex scroll-based animations powered by Flutter. The app showcases advanced animation techniques and responsive design across multiple platforms including Android, iOS, Windows, MacOS, Linux, and Web. It demonstrates Flutter's capability for creating sophisticated user interfaces with smooth animations.".to_string(),
            tags: vec!["macos".to_string(), "concept".to_string(), "flutter".to_string(), "hacktoberfest".to_string(), "flutter-examples".to_string(), "flutter-animation".to_string(), "flutter-desktop".to_string(), "hacktoberfest2020".to_string(), "aneesh".to_string()],
            clean_tags: vec!["flutter".to_string(), "concept".to_string(), "flutter-animation".to_string()],
            stars: 61,
            forks: 18,
            icon: "▶".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/invmovieconcept1".to_string());
                links.insert("demo".to_string(), "https://movieui.hmziq.xyz".to_string());
                links.insert("youtube".to_string(), "https://www.youtube.com/watch?v=kdmr9IWAFro".to_string());
                links
            },
            number: 2,
        },
        Project {
            title: "React Native Loop Game".to_string(),
            short_description: "Infinity loop puzzle game clone built with React Native".to_string(),
            description: "A clone of the popular infinity loop puzzle game built with React Native and TypeScript. The project features game animations and puzzle mechanics implemented using React hooks. The repository is archived with plans to rewrite it in Flutter.".to_string(),
            tags: vec!["react".to_string(), "react-native".to_string(), "puzzle-game".to_string(), "react-native-game".to_string(), "react-native-animation".to_string(), "react-hooks".to_string()],
            clean_tags: vec!["react-native".to_string(), "puzzle-game".to_string(), "react-hooks".to_string()],
            stars: 22,
            forks: 13,
            icon: "∞".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/react-native-loop-game".to_string());
                links.insert("demo".to_string(), "https://rnloop.hmziq.xyz".to_string());
                links
            },
            number: 3,
        },
        Project {
            title: "Flutter Experiments".to_string(),
            short_description: "Experimental Flutter project for testing various Flutter features".to_string(),
            description: "A collection of Flutter experiments and testing implementations written primarily in Dart. The project serves as a sandbox for exploring Flutter's capabilities and testing various features and implementations.".to_string(),
            tags: vec!["flutter".to_string(), "dart".to_string(), "experimental".to_string()],
            clean_tags: vec!["flutter".to_string(), "dart".to_string(), "experimental".to_string()],
            stars: 15,
            forks: 2,
            icon: "◎".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/Flutter-Experiments".to_string());
                links.insert("flutter-docs".to_string(), "https://flutter.io/".to_string());
                links
            },
            number: 4,
        },
        Project {
            title: "RuxLog Frontend".to_string(),
            short_description: "Multi-version frontend project with TypeScript and Rust components".to_string(),
            description: "A frontend application with multiple legacy and current versions, built primarily with TypeScript (57%) and Rust (32.4%). Features both admin and client components with deployment automation scripts.".to_string(),
            tags: vec!["typescript".to_string(), "rust".to_string(), "frontend".to_string(), "web".to_string()],
            clean_tags: vec!["typescript".to_string(), "frontend".to_string(), "web".to_string()],
            stars: 2,
            forks: 0,
            icon: "◆".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/ruxlog-frontend".to_string());
                links
            },
            number: 5,
        },
        Project {
            title: "Go Cross Screenshot".to_string(),
            short_description: "Cross-platform Go CLI tool for taking screenshots".to_string(),
            description: "A lightweight Go command-line program for capturing screenshots across Windows, Linux, and macOS platforms. Designed specifically for automation testing of GUI applications with cross-platform compatibility.".to_string(),
            tags: vec!["golang".to_string(), "cross-platform".to_string(), "screenshot-capture".to_string()],
            clean_tags: vec!["golang".to_string(), "cross-platform".to_string(), "screenshot-capture".to_string()],
            stars: 2,
            forks: 1,
            icon: "◊".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/go-cross-screenshot".to_string());
                links
            },
            number: 6,
        },
        Project {
            title: "FHGLLAUNCH".to_string(),
            short_description: "Simple Dart command-line application with Flutter integration".to_string(),
            description: "A minimalist command-line application built with 100% Dart, requiring Dart 12.2+ and Flutter pub global activation. Distributed under MIT license with cross-platform installation support.".to_string(),
            tags: vec!["dart".to_string(), "cli".to_string(), "flutter".to_string()],
            clean_tags: vec!["dart".to_string(), "flutter".to_string(), "cli".to_string()],
            stars: 2,
            forks: 0,
            icon: "⟁".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/fhgllaunch".to_string());
                links
            },
            number: 7,
        },
        Project {
            title: "Personal CV".to_string(),
            short_description: "Modern responsive CV website built with Rust Dioxus SSG".to_string(),
            description: "A personal CV/portfolio website built with Rust and Dioxus framework featuring static site generation. Showcases modern web development with Rust, responsive design, and custom icon components for professional presentation.".to_string(),
            tags: vec!["rust".to_string(), "rust-ssr".to_string(), "dioxus".to_string(), "dioxus-web".to_string(), "dioxus-icons".to_string(), "rust-front".to_string(), "dioxus-ssg".to_string(), "dioxus-simple".to_string()],
            clean_tags: vec!["rust".to_string(), "dioxus".to_string(), "dioxus-web".to_string()],
            stars: 1,
            forks: 0,
            icon: "◉".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/cv".to_string());
                links.insert("live_demo".to_string(), "https://cv.hmziq.rs".to_string());
                links.insert("portfolio".to_string(), "https://hmziq.rs".to_string());
                links
            },
            number: 8,
        },
        Project {
            title: "ruxlog-backend".to_string(),
            short_description: "Rust Axum web application with authentication and blog management".to_string(),
            description: "A comprehensive web application built using Rust and Axum framework providing authentication, authorization, and blog management features. Includes user registration, login, email verification, password reset, role-based access control, and CRUD operations for posts, categories, and tags. Currently incomplete and not stable according to the developer.".to_string(),
            tags: vec!["rust".to_string(), "axum".to_string(), "backend".to_string(), "authentication".to_string()],
            clean_tags: vec!["rust".to_string(), "axum".to_string(), "backend".to_string()],
            stars: 1,
            forks: 0,
            icon: "◊".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/ruxlog-backend".to_string());
                links
            },
            number: 9,
        },
        Project {
            title: "Epic Sax Gandalf Infinite".to_string(),
            short_description: "Multi-platform Epic Sax Gandalf infinite loop application".to_string(),
            description: "A Flutter-based application featuring a perfect seamless loop of Gandalf with epic sax music, available across multiple platforms. Supports Android, iOS, Windows, Linux, macOS, and web with native video playback and synchronized audio. Created as a fun tribute to the Epic Sax Guy meme.".to_string(),
            tags: vec!["gandalf".to_string(), "flutter".to_string(), "multi-platform".to_string(), "entertainment".to_string()],
            clean_tags: vec!["flutter".to_string(), "multi-platform".to_string(), "entertainment".to_string()],
            stars: 1,
            forks: 0,
            icon: "♫".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/gandalf-sax".to_string());
                links.insert("google_play".to_string(), "https://play.google.com/store/apps/details?id=com.onemdev.gandalf".to_string());
                links.insert("web_demo".to_string(), "https://gandalf.hmziq.xyz".to_string());
                links
            },
            number: 10,
        },
        Project {
            title: "next-blog".to_string(),
            short_description: "Next.js blog application bootstrapped with create-next-app".to_string(),
            description: "A standard Next.js project created with create-next-app template for building blog applications. Uses TypeScript as primary language and follows Next.js conventions for server-side rendering and static site generation. Includes standard development commands and deployment recommendations.".to_string(),
            tags: vec!["react".to_string(), "ssr".to_string(), "nextjs".to_string(), "react-ssg".to_string()],
            clean_tags: vec!["nextjs".to_string(), "react".to_string(), "ssr".to_string()],
            stars: 1,
            forks: 0,
            icon: "◇".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/next-blog".to_string());
                links
            },
            number: 11,
        },
        Project {
            title: "go-minesweeper".to_string(),
            short_description: "CLI-based minesweeper game built with Go".to_string(),
            description: "A terminal-based implementation of the classic Minesweeper game written entirely in Go. Created as a learning project during a sleepless night, featuring simple game logic and UI interactions. Includes proper code architecture with separate modules for constants, engine, models, and UI components.".to_string(),
            tags: vec!["go".to_string(), "game".to_string(), "minesweeper".to_string(), "cli".to_string(), "terminal".to_string()],
            clean_tags: vec!["go".to_string(), "game".to_string(), "cli".to_string()],
            stars: 1,
            forks: 0,
            icon: "⬡".to_string(),
            links: {
                let mut links = HashMap::new();
                links.insert("github".to_string(), "https://github.com/hmziqrs/go-minesweeper".to_string());
                links
            },
            number: 12,
        },
    ]
}