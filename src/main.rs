use chrono::{DateTime, Datelike, NaiveDate, Utc};
use github_stats_rs::{
    domain::{
        contribution_years, contributions_by_year, repos_overview, ContributionYears,
        ContributionsByYear, ReposOverview,
    },
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

    let current_year = Utc::now().year();
    let beginning_of_year = NaiveDate::from_ymd(current_year, 1, 1).and_hms(0, 0, 0);
    let next_year = NaiveDate::from_ymd(current_year + 1, 1, 1).and_hms(0, 0, 0);

    // Convert NaiveDate to DateTime<Utc>
    let beginning_of_year_utc: DateTime<Utc> = DateTime::from_utc(beginning_of_year, Utc);
    let next_year_utc: DateTime<Utc> = DateTime::from_utc(next_year, Utc);

    // Convert DateTime<Utc> to string
    let beginning_of_year_str = beginning_of_year_utc.to_rfc3339();
    let next_year_str = next_year_utc.to_rfc3339();

    let variables_3 = contributions_by_year::Variables {
        from: beginning_of_year_str,
        to: next_year_str,
    };

    let response_body_3 = post_graphql_blocking::<ContributionsByYear, _>(
        &client,
        "https://api.github.com/graphql",
        variables_3,
    );

    tracing::info!("{response_body:#?}");
    tracing::info!("{response_body_2:#?}");
    tracing::info!("{response_body_3:#?}");

    Ok(())
}
