use crate::{
    components::{cards::ContactCard, decorative::AtherCanvas},
    ui::SectionTitle,
};
use dioxus::prelude::*;

#[component]
pub fn ContactSection() -> Element {
    let contacts = vec![
        ("⟁", "GITHUB", "https://github.com/hmziqrs"),
        ("◈", "TWITTER", "https://twitter.com/hmziqrs"),
        ("◉", "EMAIL", "mailto:hmziqrs@gmail.com"),
        ("◆", "WEBSITE", "https://www.hmziq.rs/"),
    ];

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
        }
    }
}
