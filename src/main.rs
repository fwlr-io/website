use fwlr_io::post;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "404 Not Found.">
                <ParentRoute path=path!("/") view=Outlet>
                    // <Route path=path!("/work") view=pages::HireMe />
                    <Route
                        path=path!("/ok-into-result")
                        view=post::OkIntoResult
                    />
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
                    <Route path=path!("/") view=post::All />
                    <Route
                        path=path!("/*any")
                        view=|| view! { <h1>"404 Not Found"</h1> }
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(App)
}
