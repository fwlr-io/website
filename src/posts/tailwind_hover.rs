use crate::codeblock;
use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn TailwindHover() -> impl IntoView {
    view! {
        <p>
            r##"In Tailwind v4, "##<C c="hover" />r##"
            wasn't behaving as I expected.
            After some digging, I found the solution."##
        </p>
        <p>
            r##"When using the hover variant, Tailwind will compile/generate this:"##
        </p>
        <codeblock::TailwindProblem />
        <p>
            r##"I gather the intent is to exempt mobile devices, where the hover state
            would activate after interacting, and persist until the next interaction."##
        </p>
        <p>
            r##"However, the exemption is over-broad, disabling hover for various devices where hover
            is expected to work - tablets, touchscreen laptops, Safari Technology Preview on my MacBook Pro."##
        </p>
        <p>
            r##"The solution is to override the default variant with your own:"##
        </p>
        <codeblock::TailwindSolution />
        <p>
            r##"Put that in your Tailwind config (after importing Tailwind, before other configuration). Tailwind will then give you a less surprising output:"##
        </p>
        <codeblock::TailwindResult />
    }
}
