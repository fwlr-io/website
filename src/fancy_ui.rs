use leptos::prelude::*;
use leptos_use::{UseClipboardOptions, UseClipboardReturn, use_clipboard_with_options};
use tailwind_fuse::*;

/// With credit to jarthod: https://gist.github.com/jarthod/8719db9fef8deb937f4f
#[component]
pub fn FancyBrowser(children: Children) -> impl IntoView {
    view! {
        <div class="relative self-center border shadow-md bg-[rgba(250,250,250,0.7)] min-w-xl aspect-4/3 border-t-[2em] border-black/50 rounded-2.5 shadow-black/50 before:absolute before:block before:-top-[1.25em] before:left-[1em] before:w-[0.5em] before:h-[0.5em] before:rounded-[50%] before:bg-red before:[box-shadow:0_0_0_2px_var(--color-red),1.5em_0_0_2px_var(--color-yellow),3em_0_0_2px_var(--color-green)]">
            {children()}
        </div>
    }
}

#[component]
pub fn FancyCode(
    raw: &'static str,
    code: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str,
) -> impl IntoView {
    view! {
        <div class=tw_merge!(
            "relative self-center font-mono border border-black/25 p-r rounded-md min-w-xl",
            "bg-black bg-linear from-black from-20% via-dim-white/8 via-50% to-black to-80%",
            "shadow-lg text-shadow-sm/30 shadow-dim-grey inset-shadow-sm inset-shadow-white/20",
            container_class
        )>
            <div
                class=tw_merge!(
                    "overflow-x-scroll w-full font-mono whitespace-pre",
                    "text-dim-white text-shadow-sm/30 my-rounded-correct",
                    class
                )
                inner_html=code
            />
            <Raw code=raw />
        </div>
    }
}

#[component]
pub fn Raw(code: &'static str) -> impl IntoView {
    let UseClipboardReturn { copied, copy, .. } =
        use_clipboard_with_options(UseClipboardOptions::default().copied_reset_delay(3000.0));

    view! {
        <button
            class="absolute top-2.5 right-2.5 cursor-pointer pointer-events-auto rounded-2.5 border-grey border-1 hover:bg-dim-grey"
            class=("border-dim-green!", copied)
            on:click={
                let copy = copy.clone();
                move |_| copy(&code)
            }
        >
            <Show
                when=copied
                fallback=move || {
                    view! {
                        <p class="p-2.5 text-sm text-grey my-rounded-correct hover:text-dim-white">
                            "COPY"
                        </p>
                    }
                }
            >
                <p class="p-2.5 text-sm text-dim-green my-rounded-correct">
                    "DONE"
                </p>
            </Show>
        </button>
    }
}
