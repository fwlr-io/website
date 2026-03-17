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
                // <Route path=path!("/work") view=pages::HireMe />
                <ParentRoute path=path!("/post") view=pages::Post>
                    <Route
                        path=path!("/tailwind-hover")
                        view=posts::TailwindHover
                    />
                    <Route
                        path=path!("/modern-terminal")
                        view=posts::ModernTerminal
                    />
                    // <Route
                    // path=path!("/state-of-intent")
                    // view=posts::StateOfIntent
                    // />
                    <Route
                        path=path!("")
                        view=posts::All
                    />
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
