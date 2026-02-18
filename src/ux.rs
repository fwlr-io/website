use leptos::prelude::*;
use tailwind_fuse::*;

#[component]
pub fn PlainCode(
    raw: &'static str,
    code: &'static str,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str,
) -> impl IntoView {
    view! {
        <div class=tw_merge!(
            "flex justify-center bg-black border py-1g border-y-grey not-last:mb-3g", container_class
        )>
            <div class="sr-only">{raw}</div>
            <div
                class=tw_merge!(
                    "font-mono whitespace-pre w-xl text-dim-white", class
                )
                inner_html=code
            />
        </div>
    }
}
