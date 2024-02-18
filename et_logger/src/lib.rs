use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// Initializes the application-wide logger with the specified verbosity level.
pub fn init_logger() {
    // Create a subscriber that logs to stdout.
    let subscriber = FmtSubscriber::builder()
        // Customize the subscriber's behavior
        // For example, set the max level to debug and choose a compact format
        .with_max_level(Level::DEBUG)
        .with_target(false) // If you prefer not to include the target in the logs
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
