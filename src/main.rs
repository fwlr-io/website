use leptos::mount::mount_to_body;
use tracing::info;
use tracing_subscriber::fmt::format;
use tracing_subscriber_wasm::MakeConsoleWriter;

mod app;
use app::App;

fn main() {
    tracing_subscriber::fmt()
        .event_format(
            format()
                .with_ansi(false)
                .with_target(false)
                .with_level(false)
                .compact(),
        )
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        .without_time()
        .init();
    console_error_panic_hook::set_once();

    info!("mounting");

    mount_to_body(App)
}
