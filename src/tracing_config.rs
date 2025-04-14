use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    // Try to get log level from env, fallback to "info" if not set
    let filter = EnvFilter::from_default_env().add_directive("info".parse().unwrap());

    fmt()
        .with_env_filter(filter)
        .with_target(true) // shows crate/module path in logs
        .with_line_number(true) // shows line number in log output
        .compact() // cleaner formatting (feel free to use pretty() for multiline)
        .init();
}
