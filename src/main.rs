use github_stats_rs::{
    algebra::{GithubExt, ImageGenExt},
    service::{Configuration, Github, ImageGen, Telemetry},
};
use reqwest::blocking::Client;
use secrecy::ExposeSecret;
use tracing_subscriber::Registry;

fn main() -> Result<(), anyhow::Error> {
    Telemetry::<Registry>::new("github_stats_rs".into(), "info".into(), std::io::stdout).init();

    let configuration = Configuration::load_or_die();
    tracing::info!("{configuration:#?}");

    let client = Client::builder()
        .user_agent("graphql-rust")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!(
                    "Bearer {}",
                    configuration.access_token().expose_secret()
                ))
                .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let github = Github::new(configuration.clone(), client);
    let stats = github.get_stats()?;
    let lines_changed = stats.lines_changed();
    let total_contributions = stats.total_contributions();

    println!("Lines changed: {}, {}", lines_changed.0, lines_changed.1);
    println!("Total contributions: {}", total_contributions);

    // Generate the images
    let image_gen = ImageGen::new(
        configuration.template_folder().to_string(),
        configuration.output_folder().to_string(),
    );
    image_gen.generate_overview(&stats)?;
    image_gen.generate_languages(&stats)?;
    image_gen.generate_contributions_grid(&stats)?;

    tracing::info!("Total contributions: {}", total_contributions);
    Ok(())
}
