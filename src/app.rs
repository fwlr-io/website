use crate::pages;
use crate::posts;
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 Not Found.">
                <Route path=path!("/") view=pages::Home />
                <ParentRoute path=path!("/p") view=pages::Posts>
                    <Route
                        path=path!("tailwind-hover")
                        view=posts::TailwindHover
                    />
                </ParentRoute>
                <Route
                    path=path!("/*any")
                    view=|| view! { <h1>"404 Not Found"</h1> }
                />
            </Routes>
        </Router>
    }
}
