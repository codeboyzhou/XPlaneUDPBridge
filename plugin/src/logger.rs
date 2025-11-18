use crate::plugin;
use chrono::Local;
use tracing::info;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::writer::BoxMakeWriter;

struct LocalTime;

impl FormatTime for LocalTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub fn init() {
    let filename = plugin::NAME.to_string() + ".log";
    let file = std::fs::File::create(filename).unwrap();
    let writer = BoxMakeWriter::new(file);
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_timer(LocalTime)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("logger initialized");
}
