use crate::content::Content;
use leptos::prelude::*;
use leptos_mview::mview;

#[component]
pub fn App() -> impl IntoView {
    mview! { Content; }
}
