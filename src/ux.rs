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
            "flex justify-center bg-black border py-nr border-y-grey not-last:mb-fr", container_class
        )>
            <div class="sr-only">{raw}</div>
            <div
                class=tw_merge!(
                    "font-mono text-sm whitespace-pre w-xl text-dim-white", class
                )
                inner_html=code
            />
        </div>
    }
}

#[component]
pub fn P(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <p class=tw_merge!(
            "self-center font-serif text-justify text-md w-xl", class
        )>{children()}</p>
    }
}

#[component]
pub fn C(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! {
        <span class=tw_merge!(
            "font-mono text-base rounded-sm py-vnr px-nr bg-dim-grey text-yellow", class
        )>{children()}</span>
    }
}
