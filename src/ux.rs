use leptos::prelude::*;
use leptos_use::{UseClipboardOptions, UseClipboardReturn, use_clipboard_with_options};

#[component]
pub fn Code(children: Children) -> impl IntoView {
    view! {
        <div class="relative p-1 bg-black to-black shadow-lg border-[0.5px] border-black/25 bg-radial-[at_10%_0%] to-20% from-dim-white/15 shadow-dim-grey rounded-1 inset-shadow-sm inset-shadow-white/20">
            {children()}
        </div>
    }
}

#[component]
pub fn Raw(code: String) -> impl IntoView {
    let UseClipboardReturn { copied, copy, .. } =
        use_clipboard_with_options(UseClipboardOptions::default().copied_reset_delay(3000.0));

    view! {
        <Show
            when=copied
            fallback=move || {
                view! {
                    <button
                        class="absolute top-1 right-1 px-1 font-mono text-sm border rounded-1 border-grey"
                        on:click={
                            let copy = copy.clone();
                            let code = code.clone();
                            move |_| copy(&code)
                        }
                    >
                        <p class="py-1 text-grey hover:text-dim-white">"copy"</p>
                    </button>
                }
            }
        >

            <button class="absolute top-1 right-1 px-1 font-mono text-sm border rounded-1 border-dim-green">
                <p class="py-1 text-dim-green">"done"</p>
            </button>
        </Show>
    }
}

#[component]
pub fn Line(#[prop(default = 0)] indent: usize, children: Children) -> impl IntoView {
    view! {
        <p class="font-mono" data-indent=indent>
            <span class="whitespace-pre">{" ".repeat(indent)}</span>
            {children()}
        </p>
    }
}
