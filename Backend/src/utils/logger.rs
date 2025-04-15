use tracing_subscriber::{
    fmt,
    fmt::time::ChronoLocal,
};
use tracing::Level;

pub fn init() {
    let log_event = fmt::format()
        .with_target(false)
        .with_source_location(false);

    let log_time = ChronoLocal::new("[%d-%b-%Y %H:%M:%S]".to_string());

    let subscriber = fmt()
        .compact()
        .event_format(log_event)
        .with_timer(log_time)
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("[!] Tracing subscriber failed to initialize");
}
