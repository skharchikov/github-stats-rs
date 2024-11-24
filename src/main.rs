use github_stats_rs::service::{Configuration, Telemetry};
use tracing_subscriber::Registry;

fn main() {
    Telemetry::<Registry>::new("github_stats_rs".into(), "info".into(), std::io::stdout).init();

    let configuration = Configuration::load_or_die();
    tracing::info!("{configuration:#?}");
    println!("Hello, world!");
}
