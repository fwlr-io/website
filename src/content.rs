use leptos::prelude::*;
use leptos_mview::mview;

// todo: combine Exposition and Details
// todo: just delete Summary, Exposition, Details

#[component]
pub fn Summary(children: Children) -> impl IntoView {
    mview! {
        div #summary class="h-full w-1/2 flex-none overflow-y-scroll overscroll-y-contain" (
            {children()}
        )
    }
}

#[component]
pub fn Exposition(children: Children) -> impl IntoView {
    mview! {
        div #exposition class="h-full w-1/2 flex-none overflow-y-scroll overscroll-y-contain py-[50vh]" (
            {children()}
        )
    }
}

#[component]
pub fn Details(children: Children) -> impl IntoView {
    mview! {
        div #details class="h-full w-1/2 flex-none overflow-y-scroll overscroll-y-contain py-[50vh]" (
            {children()}
        )
    }
}

// todo:
//  some way to pass in one string, e.g. "pharetra"
//  and get back Pharetra::Anchor, Pharetra::Section / Pharetra::Aside
// HOCs? slots? a struct?
// Anchor<'pharetra'> ?

#[component]
pub fn Anchor(text: &'static str) -> impl IntoView {
    mview! {
        a href=f["#{text}"] (
            {text}
        )
    }
}

#[component]
pub fn Section(anchor: &'static str, children: Children) -> impl IntoView {
    mview! {
        section id={anchor} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
            {children()}
        )
    }
}

#[component]
pub fn Aside(anchor: &'static str, children: Children) -> impl IntoView {
    mview! {
        aside id={anchor} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
            {children()}
        )
    }
}
