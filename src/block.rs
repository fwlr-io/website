
pub mod modern_terminal {
    use leptos::prelude::*;

    #[component]
    pub fn Rg() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_rg.hlhtml") />
        }
    }

    #[component]
    pub fn RgReplace() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_rg-replace.hlhtml") />
        }
    }

    #[component]
    pub fn DeltaConfig() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_delta-config.hlhtml") />
        }
    }

    #[component]
    pub fn DiffLess() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_diff-less.hlhtml") />
        }
    }

    #[component]
    pub fn Grep() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_grep.hlhtml") />
        }
    }

    #[component]
    pub fn RgPcre() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_rg-pcre.hlhtml") />
        }
    }

    #[component]
    pub fn FzfFs() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fzf-fs.hlhtml") />
        }
    }

    #[component]
    pub fn Fd() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fd.hlhtml") />
        }
    }

    #[component]
    pub fn EzaLong() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_eza-long.hlhtml") />
        }
    }

    #[component]
    pub fn RgRegex() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_rg-regex.hlhtml") />
        }
    }

    #[component]
    pub fn Cat() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_cat.hlhtml") />
        }
    }

    #[component]
    pub fn DiffDelta() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_diff-delta.hlhtml") />
        }
    }

    #[component]
    pub fn InstallRg() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_install-rg.hlhtml") />
        }
    }

    #[component]
    pub fn Find() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_find.hlhtml") />
        }
    }

    #[component]
    pub fn FzfJq() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fzf-jq.hlhtml") />
        }
    }

    #[component]
    pub fn EzaShort() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_eza-short.hlhtml") />
        }
    }

    #[component]
    pub fn GrepRegex() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_grep-regex.hlhtml") />
        }
    }

    #[component]
    pub fn Z() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_z.hlhtml") />
        }
    }

    #[component]
    pub fn InstallFzf() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_install-fzf.hlhtml") />
        }
    }

    #[component]
    pub fn InstallFd() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_install-fd.hlhtml") />
        }
    }

    #[component]
    pub fn InstallDelta() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_install-delta.hlhtml") />
        }
    }

    #[component]
    pub fn FzfConfig() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fzf-config.hlhtml") />
        }
    }

    #[component]
    pub fn LsLong() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_ls-long.hlhtml") />
        }
    }

    #[component]
    pub fn InstallBat() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_install-bat.hlhtml") />
        }
    }

    #[component]
    pub fn FzfHist() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fzf-hist.hlhtml") />
        }
    }

    #[component]
    pub fn EzaConfig() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_eza-config.hlhtml") />
        }
    }

    #[component]
    pub fn FzfGitBat() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fzf-git-bat.hlhtml") />
        }
    }

    #[component]
    pub fn Bat() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_bat.hlhtml") />
        }
    }

    #[component]
    pub fn LsShort() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_ls-short.hlhtml") />
        }
    }

    #[component]
    pub fn FzfCd() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fzf-cd.hlhtml") />
        }
    }

    #[component]
    pub fn FzfGitDelta() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/modern_terminal_fzf-git-delta.hlhtml") />
        }
    }
}

pub mod tailwind_hover {
    use leptos::prelude::*;

    #[component]
    pub fn Solution() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/tailwind_hover_solution.hlhtml") />
        }
    }

    #[component]
    pub fn Problem() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/tailwind_hover_problem.hlhtml") />
        }
    }

    #[component]
    pub fn Edit() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/tailwind_hover_edit.hlhtml") />
        }
    }

    #[component]
    pub fn Result() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/tailwind_hover_result.hlhtml") />
        }
    }
}

pub mod okay_try {
    use leptos::prelude::*;

    #[component]
    pub fn TrackCaller() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_track-caller.hlhtml") />
        }
    }

    #[component]
    pub fn ImplOptionExt() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_impl-option-ext.hlhtml") />
        }
    }

    #[component]
    pub fn UsefulErrorDebug() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_useful-error-debug.hlhtml") />
        }
    }

    #[component]
    pub fn OptionExtTrackCaller() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_option-ext-track-caller.hlhtml") />
        }
    }

    #[component]
    pub fn HelperFunction() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_helper-function.hlhtml") />
        }
    }

    #[component]
    pub fn OkAnyhow() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_ok-anyhow.hlhtml") />
        }
    }

    #[component]
    pub fn HelperToOption() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_helper-to-option.hlhtml") />
        }
    }

    #[component]
    pub fn OkIntoResult() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_ok-into-result.hlhtml") />
        }
    }

    #[component]
    pub fn UsefulErrorDisplay() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_useful-error-display.hlhtml") />
        }
    }

    #[component]
    pub fn NoneError() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_none-error.hlhtml") />
        }
    }

    #[component]
    pub fn ReadToOption() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_read-to-option.hlhtml") />
        }
    }

    #[component]
    pub fn ImplOption() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("blocks/okay_try_impl-option.hlhtml") />
        }
    }
}
