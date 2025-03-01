use tracing::{Level, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
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

    println!("------------------ this is a line ------------------");

    let subscriber2 = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    {
        let _scope = tracing::subscriber::set_default(subscriber2);
        info!("This will be logged to stdout");
    }
    info!("This will _not_ be logged to stdout");

    println!("------------------ this is a line ------------------");

    log::info!("This will _not_ be logged to stdout");
    tracing::info!("This will _not_ be logged to stdout");
    {
        let layered = tracing_subscriber::registry().with(fmt::layer().with_thread_names(true));
        let _scope = tracing::subscriber::set_default(layered);
        log::info!("This will _not_ be logged to stdout");
        tracing::trace!("This will be logged to stdout");
    }
    println!("------------------ this is a line ------------------");
    {
        let layered = tracing_subscriber::registry().with(fmt::layer().with_thread_names(true));
        let _scope = layered.set_default();
        log::info!("This will be logged to stdout");
        tracing::trace!("This will be logged to stdout");
    }
    log::info!("This will _not_ be logged to stdout");
    tracing::trace!("This will _not_ be logged to stdout");
}
