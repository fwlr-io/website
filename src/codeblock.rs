use leptos::prelude::*;
use crate::ux::CodeBox;

#[component]
pub fn ModernTerminalDeltaConfig() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_delta-config.toml")
            code=include_str!("codeblocks/modern_terminal_delta-config.hlt")
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
pub fn ModernTerminalInstallRg() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_install-rg.sh")
            code=include_str!("codeblocks/modern_terminal_install-rg.hlt")
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
pub fn ModernTerminalInstallFzf() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_install-fzf.sh")
            code=include_str!("codeblocks/modern_terminal_install-fzf.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalInstallFd() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_install-fd.sh")
            code=include_str!("codeblocks/modern_terminal_install-fd.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalInstallDelta() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_install-delta.sh")
            code=include_str!("codeblocks/modern_terminal_install-delta.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFzfConfig() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_fzf-config.sh")
            code=include_str!("codeblocks/modern_terminal_fzf-config.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalInstallBat() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_install-bat.sh")
            code=include_str!("codeblocks/modern_terminal_install-bat.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalEzaConfig() -> impl IntoView {
    view! {
        <CodeBox
            raw=include_str!("codeblocks/modern_terminal_eza-config.sh")
            code=include_str!("codeblocks/modern_terminal_eza-config.hlt")
        />
    }
}
