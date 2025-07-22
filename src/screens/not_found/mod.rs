use dioxus::prelude::*;

#[component]
pub fn NotFoundScreen(route: Vec<String>) -> Element {
    let attempted_path = route.join("/");

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-purple-400 via-pink-500 to-red-500 flex items-center justify-center px-4",
            div { class: "max-w-lg mx-auto text-center",
                // 404 Number
                div { class: "mb-8",
                    h1 { class: "text-9xl font-extrabold text-white opacity-80 mb-4",
                        "404"
                    }
                    div { class: "w-32 h-1 bg-white mx-auto rounded-full opacity-60" }
                }

                // Error Message
                div { class: "mb-8",
                    h2 { class: "text-3xl md:text-4xl font-bold text-white mb-4",
                        "Oops! Page Not Found"
                    }
                    p { class: "text-lg text-white opacity-90 mb-2",
                        "The page you're looking for doesn't exist."
                    }
                    if !attempted_path.is_empty() {
                        p { class: "text-sm text-white opacity-70 font-mono bg-black bg-opacity-20 px-3 py-1 rounded-md inline-block",
                            "/{attempted_path}"
                        }
                    }
                }

                // Action Buttons
                div { class: "space-y-4 sm:space-y-0 sm:space-x-4 sm:flex sm:justify-center",
                    Link {
                        to: crate::router::Route::HomeScreen {},
                        class: "inline-block bg-white text-purple-600 font-semibold px-8 py-3 rounded-lg shadow-lg hover:bg-gray-100 transform hover:scale-105 transition-all duration-200",
                        "Go Home"
                    }
                    button {
                        class: "inline-block bg-transparent border-2 border-white text-white font-semibold px-8 py-3 rounded-lg hover:bg-white hover:text-purple-600 transform hover:scale-105 transition-all duration-200",
                        onclick: move |_| {
                            if let Some(history) = web_sys::window().and_then(|w| w.history().ok()) {
                                let _ = history.back();
                            }
                        },
                        "Go Back"
                    }
                }

                // Decorative Elements
                div { class: "mt-12 flex justify-center space-x-4",
                    div { class: "w-3 h-3 bg-white rounded-full opacity-60 animate-bounce" }
                    div { class: "w-3 h-3 bg-white rounded-full opacity-60 animate-bounce animation-delay-100" }
                    div { class: "w-3 h-3 bg-white rounded-full opacity-60 animate-bounce animation-delay-200" }
                }
            }

            // Background Decoration
            div { class: "absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none",
                // Floating shapes for visual interest
                div { class: "absolute top-1/4 left-1/4 w-32 h-32 bg-white opacity-10 rounded-full animate-pulse" }
                div { class: "absolute bottom-1/4 right-1/4 w-24 h-24 bg-white opacity-10 rounded-full animate-pulse animation-delay-300" }
                div { class: "absolute top-3/4 left-1/3 w-16 h-16 bg-white opacity-10 rounded-full animate-pulse animation-delay-150" }
            }
        }
    }
}
