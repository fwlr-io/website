use crate::ux::CodeBox;
use leptos::prelude::*;

#[component]
pub fn InstallFzf() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/install-fzf.sh")
            code=include_str!("codeblocks/install-fzf.hlt")
        />
    }
}

#[component]
pub fn InstallDelta() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/install_delta.sh")
            code=include_str!("codeblocks/install_delta.hlt")
        />
    }
}

#[component]
pub fn FzfConfig() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/fzf-config.sh")
            code=include_str!("codeblocks/fzf-config.hlt")
        />
    }
}

#[component]
pub fn EzaConfig() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/eza_config.sh")
            code=include_str!("codeblocks/eza_config.hlt")
        />
    }
}

#[component]
pub fn InstallFd() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/install-fd.sh")
            code=include_str!("codeblocks/install-fd.hlt")
        />
    }
}

#[component]
pub fn TailwindSolution() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/tailwind_solution.css")
            code=include_str!("codeblocks/tailwind_solution.hlt")
        />
    }
}

#[component]
pub fn TailwindProblem() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/tailwind_problem.css")
            code=include_str!("codeblocks/tailwind_problem.hlt")
        />
    }
}

#[component]
pub fn TailwindEdit() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/tailwind_edit.sh")
            code=include_str!("codeblocks/tailwind_edit.hlt")
        />
    }
}

#[component]
pub fn TailwindResult() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/tailwind_result.css")
            code=include_str!("codeblocks/tailwind_result.hlt")
        />
    }
}

#[component]
pub fn InstallBat() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/install_bat.sh")
            code=include_str!("codeblocks/install_bat.hlt")
        />
    }
}

#[component]
pub fn DeltaConfig() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/delta_config.toml")
            code=include_str!("codeblocks/delta_config.hlt")
        />
    }
}

#[component]
pub fn InstallRg() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/install-rg.sh")
            code=include_str!("codeblocks/install-rg.hlt")
        />
    }
}
