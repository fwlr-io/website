use crate::posts;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! { <posts::ModernTerminal /> }
}
