pub mod okay_try {
    use leptos::prelude::*;

    #[component]
    pub fn QuestionMarkCorrect() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("custom_blocks/okay_try_use-of-question-mark.hlhtml") />
        }
    }

    #[component]
    pub fn QuestionMarkWrongParents() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("custom_blocks/okay_try_question-mark-wrong-parents.hlhtml") />
        }
    }

    #[component]
    pub fn QuestionMarkOk() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("custom_blocks/okay_try_question-mark-ok.hlhtml") />
        }
    }

    #[component]
    pub fn QuestionMarkOkOr() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("custom_blocks/okay_try_question-mark-ok-or.hlhtml") />
        }
    }

    #[component]
    pub fn InherentImpl() -> impl IntoView {
        view! {
            <crate::ux::SourceBox hlt=include_str!("custom_blocks/okay_try_inherent-impl.hlhtml") />
        }
    }
}
