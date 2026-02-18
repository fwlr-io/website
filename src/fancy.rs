use leptos::prelude::*;
use leptos_use::{UseClipboardOptions, UseClipboardReturn, use_clipboard_with_options};
use tailwind_fuse::*;

#[component]
pub fn FancyCode(
    raw: &'static str,
    code: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str,
) -> impl IntoView {
    view! {
        <div class=tw_merge!(
            "relative self-center font-mono bg-black from-black to-black border shadow-lg from-20% to-80% text-shadow-sm/30 min-w-xl p-1g border-black/25 bg-linear-150 via-dim-white/8 via-50% shadow-dim-grey rounded-1g inset-shadow-sm inset-shadow-white/20", container_class
        )>
            <div
                class=tw_merge!(
                    "overflow-x-scroll w-full font-mono whitespace-pre text-dim-white text-shadow-sm/30 my-rounded-correct", class
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
