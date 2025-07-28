use crate::components::buttons::ProjectLink;
use crate::types::Project;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectCardProps {
    project: Project,
}

#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    rsx! {
        div {
            class: "border border-white/20 p-10 transition-all duration-500 ease-in-out relative overflow-hidden bg-black/50 backdrop-blur-sm hover:translate-y-[-10px] hover:shadow-[0_20px_40px_rgba(255,255,255,0.1)] hover:border-white/50 opacity-100 translate-y-0 group",

            // Project number overlay
            div {
                class: "absolute top-8 right-8 text-8xl font-thin opacity-10 pointer-events-none",
                "{props.project.number:02}"
            }

            // Icon with pulse animation
            div {
                class: "text-6xl mb-8 animate-pulse",
                "{props.project.icon}"
            }

            // Title
            h3 {
                class: "text-2xl font-light mb-4 tracking-[0.1rem]",
                "{props.project.title}"
            }

            // Stars and Forks
            div {
                class: "flex gap-4 mb-4 text-xs opacity-60",
                span {
                    class: "flex items-center gap-1",
                    "⭐ {props.project.stars}"
                }
                if props.project.forks > 0 {
                    span {
                        class: "flex items-center gap-1",
                        "🍴 {props.project.forks}"
                    }
                }
            }

            // Short Description
            p {
                class: "text-sm opacity-70 mb-8 leading-relaxed",
                "{props.project.short_description}"
            }

            // Tags
            div {
                class: "flex flex-wrap gap-3 mb-8",
                for tag in props.project.clean_tags.iter() {
                    span {
                        class: "px-4 py-1.5 border border-white/30 text-xs opacity-60 uppercase tracking-[0.1rem] transition-all duration-300 ease-in-out hover:opacity-100 hover:border-white",
                        "{tag}"
                    }
                }
            }

            // Links
            div {
                class: "flex gap-4 flex-wrap",
                for (link_type, url) in props.project.links.iter() {
                    ProjectLink {
                        text: link_type.to_uppercase(),
                        href: url.clone()
                    }
                }
            }

            // Shine effect on hover
            div {
                class: "absolute inset-0 bg-gradient-to-r from-transparent via-white/8 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-600 ease-out transform skew-x-12 -z-10"
            }

            // Radial gradient glow on hover (positioned behind content)
            div {
                class: "absolute inset-0 bg-gradient-radial from-white/5 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10"
            }
        }
    }
}
