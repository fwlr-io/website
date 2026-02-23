use crate::codeblock;
use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <P>
            r##"In Tailwind v4, "##<C>hover:</C>r##"
            wasn't behaving as I expected.
            After some digging, I found the solution."##
        </P>
        <P>
            r##"When using the hover variant, Tailwind will compile/generate this:"##
        </P>
        <codeblock::TailwindProblem />
        <P>
            r##"I gather the intent is to exempt mobile devices, where the hover state
            would activate after interacting, and persist until the next interaction."##
        </P>
        <P>
            r##"However, the exemption is over-broad, disabling hover for various devices where hover
            is expected to work - tablets, touchscreen laptops, Safari Technology Preview on my MacBook Pro."##
        </P>
        <P>
            r##"The solution is to override the default variant with your own:"##
        </P>
        <codeblock::TailwindSolution />
        <P>
            r##"Put that in your Tailwind config, and Tailwind will give you a less surprising output:"##
        </P>
        <codeblock::TailwindResult />
        <P>
            r##"Now I just have to figure out what accursed selector is generating all this line noise..."##
        </P>
        <codeblock::TailwindLineNoise class="mask-b-from-30% mask-b-to-90%" />
    }
}
