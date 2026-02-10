use crate::content::Content;
use crate::counters::Counters;
use leptos::prelude::*;
use leptos_mview::mview;
// use crate::ux::*;

#[component]
pub fn App() -> impl IntoView {
    mview! {
        Content;
        Counters;
    }
}
