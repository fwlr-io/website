// use crate::posts;
use leptos::prelude::*;
use leptos_router::nested_router::Outlet;

#[component]
pub fn Post() -> impl IntoView {
    view! {
        <h1>"Post"</h1>
        <Outlet />
        <h1>"End post"</h1>
    }
}
