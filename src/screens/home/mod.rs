use dioxus::prelude::*;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold mb-6", "Welcome Home" }
            p { class: "text-gray-600",
                "This is the home screen of your Dioxus application. The project structure is now set up according to the guide!"
            }

            div { class: "mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    h3 { class: "text-xl font-semibold mb-2", "State Management" }
                    p { class: "text-gray-600", "Global state with StateFrame pattern for async operations" }
                }
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    h3 { class: "text-xl font-semibold mb-2", "Form Handling" }
                    p { class: "text-gray-600", "OxForm pattern with validation using the validator crate" }
                }
                div { class: "bg-white p-6 rounded-lg shadow-md",
                    h3 { class: "text-xl font-semibold mb-2", "Modular Structure" }
                    p { class: "text-gray-600", "Organized components, containers, screens, hooks, and services" }
                }
            }
        }
    }
}
