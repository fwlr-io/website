use leptos::prelude::*;
use leptos_mview::mview;

use crate::counters::*;

#[component]
pub fn App() -> impl IntoView {
    mview! {
        body class="font-serif" (
            Counters;
        )
    }
}
