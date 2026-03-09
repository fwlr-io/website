// use crate::codeblock;
use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn ModernTerminal() -> impl IntoView {
    view! {
        <P>
            r##"Many basic shell commands have superior counterparts. Upgrade your terminal experience."##
        </P>
        <P>r##"z > cd, eza > ls, bat > cat, delta > less, fzf > all, etc"##</P>
    }
}
