use crate::ux::TermBox;
use leptos::prelude::*;

#[component]
pub fn ModernTerminalRg(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_rg.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalRgReplace(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_rg-replace.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalDiffLess(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_diff-less.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalGrep(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_grep.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalRgPcre(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_rg-pcre.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFzfFs(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_fzf-fs.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFd(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_fd.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalEzaLong(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_eza-long.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalRgRegex(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_rg-regex.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalCat(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_cat.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalDiffDelta(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_diff-delta.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFind(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_find.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFzfJq(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_fzf-jq.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalEzaShort(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_eza-short.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalGrepRegex(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_grep-regex.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalZ(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_z.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalLsLong(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_ls-long.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFzfHist(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_fzf-hist.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFzfGitBat(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_fzf-git-bat.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalBat(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_bat.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalLsShort(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_ls-short.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFzfCd(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_fzf-cd.hlt")
        />
    }
}

#[component]
pub fn ModernTerminalFzfGitDelta(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/modern_terminal_fzf-git-delta.hlt")
        />
    }
}
