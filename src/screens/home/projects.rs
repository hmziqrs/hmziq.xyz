use crate::components::{buttons::FilterButton, cards::ProjectCard};
use crate::store::home::get_projects;
use crate::ui::SectionTitle;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectsSectionProps {
    active_filter: Signal<String>,
}

#[component]
pub fn ProjectsSection(mut props: ProjectsSectionProps) -> Element {
    let projects = get_projects();
    let filters = vec![
        "all",
        "mobile",
        "web",
        "rust",
        "flutter",
        "react-native",
        "open-source",
    ];

    let filtered_projects = use_memo(move || {
        let filter = (props.active_filter)();
        if filter == "all" {
            projects.clone()
        } else {
            projects
                .iter()
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
                class: "flex justify-center gap-5 mb-20 flex-wrap relative before:content-[''] before:absolute before:top-1/2 before:left-0 before:right-0 before:h-px before:bg-gradient-to-r before:from-transparent before:via-white/20 before:to-transparent before:-z-10",

                for filter in filters {
                    FilterButton {
                        text: filter.to_uppercase(),
                        filter: filter.to_string(),
                        active: (props.active_filter)() == filter,
                        on_click: move |f| props.active_filter.set(f)
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
