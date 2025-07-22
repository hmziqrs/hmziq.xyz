use crate::screens::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    // Public routes
    #[route("/")]
    HomeScreen {},

    // Catch all route
    #[route("/:..route")]
    NotFoundScreen { route: Vec<String> },
}
