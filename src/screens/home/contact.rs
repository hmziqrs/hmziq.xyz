use crate::{
    components::{cards::ContactCard, decorative::AtherCanvas},
    ui::SectionTitle,
};
use chrono::Datelike;
use dioxus::prelude::*;

#[component]
pub fn ContactSection() -> Element {
    let contacts = vec![
        ("⟁", "GITHUB", "https://github.com/hmziqrs"),
        ("◈", "TWITTER", "https://twitter.com/hmziqrs"),
        ("◉", "EMAIL", "mailto:hmziqrs@gmail.com"),
        ("◆", "WEBSITE", "https://www.hmziq.rs/"),
    ];

    // Get current date
    let now = chrono::Utc::now();
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let current_date = format!(
        "{}, {} {}",
        now.day(),
        months[(now.month() - 1) as usize],
        now.year()
    );
    let current_year = now.year();

    rsx! {
        section {
            id: "contact",
            class: "py-36 px-5 text-center relative overflow-hidden before:content-[''] before:absolute before:top-0 before:left-0 before:right-0 before:h-px before:bg-gradient-to-r before:from-transparent before:via-white before:to-transparent min-h-screen",

            AtherCanvas {}

            div {
                class: "mb-20 relative z-10",
                SectionTitle { text: "CONTACT" }
            }

            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-10 max-w-5xl mx-auto relative z-10",

                for (icon, label, href) in contacts {
                    ContactCard {
                        icon: icon.to_string(),
                        label: label.to_string(),
                        href: href.to_string()
                    }
                }
            }

            // Copyright section
            div {
                class: "mt-32 relative z-10",

                // Soft blurred background container
                div {
                    class: "inline-block px-8 py-6 bg-black/30 backdrop-blur-md",

                    // Copyright text
                    p {
                        class: "text-sm text-white/70 font-light tracking-wider mb-2",
                        "© {current_year} hmziqrs. Crafted with Dioxus and Rust."
                    }

                    // Updated date
                    p {
                        class: "text-xs text-white/50 font-light tracking-wider",
                        "Updated on {current_date}"
                    }
                }
            }
        }
    }
}
