use github_stats_rs::{
    domain::{contribution_years, repos_overview, ContributionYears, ReposOverview},
    service::{Configuration, Telemetry},
};
use graphql_client::reqwest::post_graphql_blocking;
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

    let variables = contribution_years::Variables {};

    let response_body = post_graphql_blocking::<ContributionYears, _>(
        &client,
        "https://api.github.com/graphql",
        variables,
    )
    .unwrap();

    let variables_2 = repos_overview::Variables {
        owned_cursor: None,
        contributed_cursor: None,
    };

    let response_body_2 = post_graphql_blocking::<ReposOverview, _>(
        &client,
        "https://api.github.com/graphql",
        variables_2,
    );

    tracing::info!("{response_body:#?}");
    tracing::info!("{response_body_2:#?}");

    Ok(())
}
