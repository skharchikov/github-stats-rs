use github_stats_rs::service::Configuration;

fn main() {
    let configuration = Configuration::load_or_die();
    tracing::info!("{configuration:#?}");
    println!("Hello, world!");
}
