use leptos::prelude::*;

#[component]
pub fn TermBox(hlt: &'static str, #[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <div class="bg-black rounded-sm border shadow-md border-black/25 px-nr py-vnr min-w-xl shadow-grey inset-shadow-sm inset-shadow-white/20">
            <div
                class="overflow-x-scroll w-full font-mono text-sm/4.5 text-dim-white"
                class=("text-xs/3.75", tiny)
                inner_html=hlt
            />
        </div>
    }
}

#[component]
pub fn CodeBox(raw: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="bg-black rounded-md border shadow-md border-black/25 p-r min-w-xl shadow-grey inset-shadow-sm inset-shadow-white/20">
            <div class="sr-only">{raw}</div>
            <div
                class="overflow-x-scroll w-full font-mono text-sm whitespace-pre text-dim-white my-rounded-correct"
                inner_html=code
            />
        </div>
    }
}

#[component]
pub fn Code(raw: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-center bg-black border border-y-grey py-nr not-last:mb-fr">
            <div class="sr-only">{raw}</div>
            <div
                class="font-mono text-sm whitespace-pre text-dim-white w-xl"
                inner_html=code
            />
        </div>
    }
}

#[component]
pub fn Term(hlt: &'static str, #[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <div class="flex justify-center bg-black border border-y-grey py-nr not-last:mb-fr">
            <div
                class="font-mono text-sm/4.5 text-dim-white min-w-xl"
                class=("text-xs/3.75", tiny)
                inner_html=hlt
            />
        </div>
    }
}

/// With credit to jarthod: https://gist.github.com/jarthod/8719db9fef8deb937f4f
#[component]
pub fn Browser(children: Children) -> impl IntoView {
    view! {
        <div class="relative self-center border shadow-md bg-[rgba(250,250,250,0.7)] min-w-xl aspect-4/3 border-t-[2em] border-black/50 rounded-2.5 shadow-black/50 before:absolute before:block before:-top-[1.25em] before:left-[1em] before:w-[0.5em] before:h-[0.5em] before:rounded-[50%] before:bg-red before:[box-shadow:0_0_0_2px_var(--color-red),1.5em_0_0_2px_var(--color-yellow),3em_0_0_2px_var(--color-green)]">
            {children()}
        </div>
    }
}

pub fn code(c: &'static str) -> impl IntoView {
    let big_ascenders = "bdfhkl".chars().any(|s| c.contains(s));
    let small_ascenders = !big_ascenders && "tij".chars().any(|s| c.contains(s));

    let big_descenders = "gyj".chars().any(|s| c.contains(s));
    let small_descenders = !big_descenders && "pq".chars().any(|s| c.contains(s));
    let no_descenders = !big_descenders && !small_descenders;

    view! {
        <span
            class="mr-px font-mono px-xvnr rounded-xs bg-dim-grey text-yellow"
            class:pt-px=small_ascenders
            class:pt-xxvnr=big_ascenders
            class:pb-px=no_descenders
            class:pb-xvnr=big_descenders
            class:pb-xxvnr=small_descenders
        >
            {c}
        </span>
    }
}

#[component]
pub fn Break() -> impl IntoView {
    view! {
        <div class="flex flex-row justify-center items-center w-lg px-lh py-sh gap-lh mx-lh my-sh hdiv">
            <span class="w-full h-[0.5px] bg-mid-black shrink" />
        </div>
    }
}

#[component]
pub fn BlackBox() -> impl IntoView {
    view! { <span class="font-mono text-4xl font-bold text-mid-black">"❒"</span> }
}
