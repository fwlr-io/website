use leptos::prelude::*;
use leptos_router::components::A;

mod modern_terminal;
mod separate_intent_and_state;
mod tailwind_hover;

pub use modern_terminal::ModernTerminal;
pub use separate_intent_and_state::SeparateIntentAndState;
pub use tailwind_hover::TailwindHover;

#[component]
pub fn All() -> impl IntoView {
    view! {
        <A href="tailwind-hover">"Tailwind Hover"</A>
        <A href="modern-terminal">"Modern Terminal"</A>
    }
}
