mod modern_terminal;
pub use modern_terminal::ModernTerminal;

mod okay_try;
pub use okay_try::OkayTry;

mod tailwind_hover;
pub use tailwind_hover::TailwindHover;

// mod bad_vibes;
// pub use bad_vibes::BadVibes;

// mod separate_intent_and_state;
// pub use separate_intent_and_state::SeparateIntentAndState;

use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn All() -> impl IntoView {
    view! {
        // <A href="separate-intent-and-state">"Separate Intent and State"</A>
        // <A href="bad-vibes">"Bad Vibes"</A>
        <A href="okay-try">"Okay Try"</A>
        <A href="modern-terminal">"Modern Terminal"</A>
        <A href="tailwind-hover">"Tailwind Hover"</A>
    }
}
