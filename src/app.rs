use crate::page;
use crate::post;
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 Not Found.">
                <Route path=path!("/") view=page::Home />
                // <Route path=path!("/work") view=pages::HireMe />
                <ParentRoute path=path!("/post") view=Outlet>
                    <Route
                        path=path!("/tailwind-hover")
                        view=post::TailwindHover
                    />
                    <Route
                        path=path!("/modern-terminal")
                        view=post::ModernTerminal
                    />
                    // <Route
                    // path=path!("/state-of-intent")
                    // view=post::StateOfIntent
                    // />
                    <Route path=path!("") view=post::All />
                </ParentRoute>
                // <ParentRoute path=path!("/tool") view=pages::Tool>
                // <Route path=path!("/flinket") view=tools::Flinket />
                // </ParentRoute>
                <Route
                    path=path!("/*any")
                    view=|| view! { <h1>"404 Not Found"</h1> }
                />
            </Routes>
        </Router>
    }
}
