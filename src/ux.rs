use leptos::prelude::*;
use leptos_use::{UseClipboardOptions, UseClipboardReturn, use_clipboard_with_options};

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
pub fn Code(code: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-center bg-black border py-1g border-y-grey">
            <div
                class="font-mono whitespace-pre w-xl text-dim-white"
                inner_html=code
            />
        </div>
    }
}

#[component]
pub fn FancyCode(raw: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="relative self-center font-mono bg-black from-black to-black border shadow-lg from-20% to-80% text-shadow-sm/30 min-w-xl p-1g border-black/25 bg-linear-150 via-dim-white/8 via-50% shadow-dim-grey rounded-1g inset-shadow-sm inset-shadow-white/20">
            <div
                class="overflow-x-scroll w-full font-mono whitespace-pre text-dim-white text-shadow-sm/30 my-rounded-correct"
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
            class="absolute cursor-pointer pointer-events-auto top-1g right-1g rounded-1g border-grey border-1 hover:bg-dim-grey"
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
                        <p class="text-sm p-1g text-grey my-rounded-correct hover:text-dim-white">
                            "COPY"
                        </p>
                    }
                }
            >
                <p class="text-sm p-1g text-dim-green my-rounded-correct">
                    "DONE"
                </p>
            </Show>
        </button>
    }
}
