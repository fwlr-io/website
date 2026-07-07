use leptos::prelude::*;
use leptos_router::components::A;

mod modern_terminal;
mod ok_into_result;
// mod separate_intent_and_state;
mod tailwind_hover;

pub use modern_terminal::ModernTerminal;
pub use ok_into_result::OkIntoResult;
// pub use separate_intent_and_state::SeparateIntentAndState;
pub use tailwind_hover::TailwindHover;

#[component]
pub fn All() -> impl IntoView {
    view! {
        <A href="ok-into-result">"Ok Into Result"</A>
        <A href="modern-terminal">"Modern Terminal"</A>
        <A href="tailwind-hover">"Tailwind Hover"</A>
    }
}
