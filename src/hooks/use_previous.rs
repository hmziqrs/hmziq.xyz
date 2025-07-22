use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

/// Hook that returns the previous value of a given value
/// 
/// # Example
/// ```rust
/// #[component]
/// fn Counter() -> Element {
///     let mut count = use_signal(|| 0);
///     let previous_count = use_previous(count.read().clone());
///
///     rsx! {
///         div {
///             "Current: {count}"
///             if let Some(prev) = previous_count {
///                 " Previous: {prev}"
///             }
///
///             button {
///                 onclick: move |_| count += 1,
///                 "Increment"
///             }
///         }
///     }
/// }
/// ```
pub fn use_previous<T>(current: T) -> Option<T>
where
    T: Clone + PartialEq + 'static,
{
    let state_ref = use_hook(|| Rc::new(RefCell::new(None::<T>)));
    let previous_value = state_ref.borrow().clone();

    use_effect(use_reactive!(|(current,)| {
        *state_ref.borrow_mut() = Some(current);
    }));

    previous_value
}

/// Hook that tracks whether a value has changed
/// 
/// # Example
/// ```rust
/// #[component] 
/// fn StatusIndicator(status: String) -> Element {
///     let has_changed = use_has_changed(status.clone());
///     
///     rsx! {
///         div {
///             class: if has_changed { "status-changed" } else { "status-stable" },
///             "Status: {status}"
///         }
///     }
/// }
/// ```
pub fn use_has_changed<T>(current: T) -> bool
where
    T: Clone + PartialEq + 'static,
{
    let previous = use_previous(current.clone());
    match previous {
        Some(prev) => prev != current,
        None => false, // First render, no change
    }
}

/// Hook that provides both previous value and change status
/// 
/// # Example
/// ```rust
/// #[component]
/// fn ValueTracker(value: i32) -> Element {
///     let (previous, has_changed) = use_previous_with_change(value);
///     
///     rsx! {
///         div {
///             "Current: {value}"
///             if let Some(prev) = previous {
///                 " Previous: {prev}"
///             }
///             if has_changed {
///                 " (Changed!)"
///             }
///         }
///     }
/// }
/// ```
pub fn use_previous_with_change<T>(current: T) -> (Option<T>, bool)
where
    T: Clone + PartialEq + 'static,
{
    let previous = use_previous(current.clone());
    let has_changed = match &previous {
        Some(prev) => prev != &current,
        None => false,
    };
    
    (previous, has_changed)
}