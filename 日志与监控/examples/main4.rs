use tracing::{info, Level};

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        info!("This will be logged to stdout");
    });
    info!("This will _not_ be logged to stdout");

    let subscriber2 = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    {
        let _scope = tracing::subscriber::set_default(subscriber2);
        info!("This will be logged to stdout");
    }
    info!("This will _not_ be logged to stdout");
    println!("------------------ this is a line ------------------");
    // let file_appender = tracing_appender::rolling::RollingFileAppender::new(
    //     tracing_appender::rolling::Rotation::HOURLY,
    //     "./logs",
    //     "prefix",
    // );
    let file_appender = tracing_appender::rolling::minutely("./logs", "prefix");
    let (non_blocking_appender, _writer_guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking_appender)
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("This will _not_ be logged to stdout, but to a file");
    log::info!("This will _not_ be logged to stdout");
    tracing::trace!("This will be logged to stdout, but to a file");
    // std::thread::sleep(std::time::Duration::from_secs(60));
}
