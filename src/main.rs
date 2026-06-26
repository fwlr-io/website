use fwlr_io::page;
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
                    <Route path=path!("/posts") view=post::All />
                    // <ParentRoute path=path!("/tool") view=pages::Tool>
                    // <Route path=path!("/flinket") view=tools::Flinket />
                    // </ParentRoute>
                    <Route path=path!("/") view=page::Home />
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
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .event_format(
                tracing_subscriber::fmt::format()
                    .with_ansi(false)
                    .with_level(false)
                    .compact(),
            )
            .with_writer(
                tracing_subscriber_wasm::MakeConsoleWriter::default()
                    .map_trace_level_to(tracing::Level::DEBUG),
            )
            .without_time()
            .init();

        console_error_panic_hook::set_once();
    };

    mount_to_body(App)
}
