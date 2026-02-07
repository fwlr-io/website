use fwlr_io::app::App;
use leptos::mount::mount_to_body;

fn main() {
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

    tracing::info!("mounting");

    mount_to_body(App)
}
