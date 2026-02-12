use leptos::prelude::*;
use leptos_use::{UseClipboardOptions, UseClipboardReturn, use_clipboard_with_options};

#[component]
pub fn Code(children: Children) -> impl IntoView {
    view! {
        <div class="relative bg-black to-black shadow-lg p-1g border-[0.5px] border-black/25 bg-radial-[at_10%_0%] to-20% from-dim-white/15 shadow-dim-grey rounded-1g inset-shadow-sm inset-shadow-white/20">
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
                        class="absolute font-mono text-sm border top-1g right-1g px-1g rounded-1g border-grey"
                        on:click={
                            let copy = copy.clone();
                            let code = code.clone();
                            move |_| copy(&code)
                        }
                    >
                        <p class="py-1g text-grey hover:text-dim-white">"copy"</p>
                    </button>
                }
            }
        >
            <button class="absolute font-mono text-sm border top-1g right-1g px-1g rounded-1g border-dim-green">
                <p class="py-1g text-dim-green">"done"</p>
            </button>
        </Show>
    }
}

#[component]
pub fn Line(#[prop(default = 0)] indent: usize, children: Children) -> impl IntoView {
    let spaces = " ".repeat(indent);
    view! {
        <p class="font-mono" data-indent=indent>
            <span class="whitespace-pre">{spaces}</span>
            {children()}
        </p>
    }
}
