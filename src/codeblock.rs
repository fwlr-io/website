use crate::ux;
use leptos::prelude::*;

#[component]
pub fn Code() -> impl IntoView {
    let code = include_str!("codeblocks/code.html");
    view! { <ux::Code code=code /> }
}

#[component]
pub fn FancyCode() -> impl IntoView {
    let code = include_str!("codeblocks/code.html");
    let raw = include_str!("codeblocks/code.rs");
    view! { <ux::FancyCode code=code raw=raw /> }
}

#[component]
pub fn Test() -> impl IntoView {
    let code = include_str!("codeblocks/test.html");
    view! { <ux::Code code=code /> }
}

#[component]
pub fn FancyTest() -> impl IntoView {
    let code = include_str!("codeblocks/test.html");
    let raw = include_str!("codeblocks/test.rs");
    view! { <ux::FancyCode code=code raw=raw /> }
}
