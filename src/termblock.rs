use crate::ux::TermBox;
use leptos::prelude::*;

#[component]
pub fn FzfHist(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/fzf-hist.hlt")
        />
    }
}

#[component]
pub fn Z(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/z.hlt")
        />
    }
}

#[component]
pub fn Bat(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/bat.hlt")
        />
    }
}

#[component]
pub fn FzfJq(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/fzf-jq.hlt")
        />
    }
}

#[component]
pub fn LsShort(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/ls-short.hlt")
        />
    }
}

#[component]
pub fn FzfFs(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/fzf-fs.hlt")
        />
    }
}

#[component]
pub fn LsLong(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/ls-long.hlt")
        />
    }
}

#[component]
pub fn Cat(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/cat.hlt")
        />
    }
}

#[component]
pub fn RgPcre(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/rg-pcre.hlt")
        />
    }
}

#[component]
pub fn Grep(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/grep.hlt")
        />
    }
}

#[component]
pub fn Fd(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/fd.hlt")
        />
    }
}

#[component]
pub fn Test(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/test.hlt")
        />
    }
}

#[component]
pub fn DiffLess(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/diff-less.hlt")
        />
    }
}

#[component]
pub fn RgReplace(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/rg-replace.hlt")
        />
    }
}

#[component]
pub fn Find(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/find.hlt")
        />
    }
}

#[component]
pub fn FzfCd(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/fzf-cd.hlt")
        />
    }
}

#[component]
pub fn GrepRegex(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/grep-regex.hlt")
        />
    }
}

#[component]
pub fn Rg(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/rg.hlt")
        />
    }
}

#[component]
pub fn EzaLong(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/eza-long.hlt")
        />
    }
}

#[component]
pub fn RgRegex(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/rg-regex.hlt")
        />
    }
}

#[component]
pub fn DiffDelta(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/diff-delta.hlt")
        />
    }
}

#[component]
pub fn FzfGitBat(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/fzf-git-bat.hlt")
        />
    }
}

#[component]
pub fn EzaShort(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/eza-short.hlt")
        />
    }
}

#[component]
pub fn FzfGitDelta(#[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <TermBox
            tiny=tiny
            hlt=include_str!("termblocks/fzf-git-delta.hlt")
        />
    }
}
