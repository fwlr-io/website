use leptos::prelude::*;

/// With credit to jarthod: https://gist.github.com/jarthod/8719db9fef8deb937f4f
#[component]
pub fn FancyBrowser(children: Children) -> impl IntoView {
    view! {
        <div class="relative self-center border shadow-md bg-[rgba(250,250,250,0.7)] min-w-xl aspect-4/3 border-t-[2em] border-black/50 rounded-1g shadow-black/50 before:absolute before:block before:-top-[1.25em] before:left-[1em] before:w-[0.5em] before:h-[0.5em] before:rounded-[50%] before:bg-red before:[box-shadow:0_0_0_2px_var(--color-red),1.5em_0_0_2px_var(--color-yellow),3em_0_0_2px_var(--color-green)]">
            {children()}
        </div>
    }
}

#[component]
pub fn P(children: Children) -> impl IntoView {
    view! {
        <p class="self-center font-serif text-justify text-md w-xl">
            {children()}
        </p>
    }
}

#[component]
pub fn C(children: Children) -> impl IntoView {
    view! {
        <span class="py-0.5 px-1.5 font-mono text-base rounded-sm bg-dim-grey text-yellow">
            {children()}
        </span>
    }
}
