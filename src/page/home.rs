use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>"Home"</h1>
        <A href="post">"Posts"</A>
    }
}
