use crate::ux;
use leptos::prelude::*;

#[component]
pub fn Code(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str
) -> impl IntoView {
    let raw = include_str!("codeblocks/code.rs");
    let code = include_str!("codeblocks/code.hhlt");

    view! { <ux::FancyCode raw=raw code=code class=class container_class=container_class /> }
}
#[component]
pub fn Test(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str
) -> impl IntoView {
    let raw = include_str!("codeblocks/test.rs");
    let code = include_str!("codeblocks/test.hhlt");

    view! { <ux::FancyCode raw=raw code=code class=class container_class=container_class /> }
}
#[component]
pub fn TailwindLineNoise(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str
) -> impl IntoView {
    let raw = include_str!("codeblocks/tailwind_line_noise.css");
    let code = include_str!("codeblocks/tailwind_line_noise.hhlt");

    view! { <ux::FancyCode raw=raw code=code class=class container_class=container_class /> }
}
#[component]
pub fn TailwindSolution(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str
) -> impl IntoView {
    let raw = include_str!("codeblocks/tailwind_solution.css");
    let code = include_str!("codeblocks/tailwind_solution.hhlt");

    view! { <ux::FancyCode raw=raw code=code class=class container_class=container_class /> }
}
#[component]
pub fn TailwindProblem(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str
) -> impl IntoView {
    let raw = include_str!("codeblocks/tailwind_problem.css");
    let code = include_str!("codeblocks/tailwind_problem.hhlt");

    view! { <ux::FancyCode raw=raw code=code class=class container_class=container_class /> }
}
#[component]
pub fn TailwindResult(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] container_class: &'static str
) -> impl IntoView {
    let raw = include_str!("codeblocks/tailwind_result.css");
    let code = include_str!("codeblocks/tailwind_result.hhlt");

    view! { <ux::FancyCode raw=raw code=code class=class container_class=container_class /> }
}