use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn All() -> impl IntoView {
    view! {
        <A href="tailwind-hover">"Tailwind Hover"</A>
        <A href="modern-terminal">"Modern Terminal"</A>
    }
}
