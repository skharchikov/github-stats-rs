use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use graphql_client::reqwest::post_graphql_blocking;

use crate::{
    algebra::GithubExt,
    domain::{
        contribution_years, contributions_by_year, repos_overview, ContributionYears,
        ContributionsByYear, ContributorActivity, Language, ReposOverview, Stats, ViewTraffic,
    },
};

use super::Configuration;

pub struct Github {
    configuration: Configuration,
    client: reqwest::blocking::Client,
    url: String,
}

impl Github {
    pub fn new(configuration: Configuration, client: reqwest::blocking::Client) -> Self {
        Self {
            configuration,
            client,
            url: "https://api.github.com".to_string(),
        }
    }

    pub fn graphql_url(&self) -> String {
        format!("{}/graphql", self.url)
    }
}

impl GithubExt for Github {
    fn total_contributions(&self) -> Result<i64, anyhow::Error> {
        let variables = contribution_years::Variables {};

        let contribution_years_response = post_graphql_blocking::<ContributionYears, _>(
            &self.client,
            &self.graphql_url(),
            variables,
        )
        .unwrap();

        let years = contribution_years_response
            .data
            .map(|data| data.viewer.contributions_collection.contribution_years);

        let by_year_response = years
            .map(|years| {
                years
                    .iter()
                    .map(|year| {
                        let year_i32 = *year as i32;
                        let beginning_of_year =
                            NaiveDate::from_ymd(year_i32, 1, 1).and_hms(0, 0, 0);
                        let next_year = NaiveDate::from_ymd(year_i32 + 1, 1, 1).and_hms(0, 0, 0);

                        // Convert NaiveDate to DateTime<Utc>
                        let beginning_of_year_utc: DateTime<Utc> =
                            DateTime::from_utc(beginning_of_year, Utc);
                        let next_year_utc: DateTime<Utc> = DateTime::from_utc(next_year, Utc);

                        // Convert DateTime<Utc> to string
                        let from = beginning_of_year_utc.to_rfc3339();
                        let to = next_year_utc.to_rfc3339();

                        let variables = contributions_by_year::Variables { from, to };
                        post_graphql_blocking::<ContributionsByYear, _>(
                            &self.client,
                            &self.graphql_url(),
                            variables,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap()
            .into_iter()
            .filter_map(|response| response.unwrap().data.map(|data| data.viewer))
            .collect::<Vec<_>>();

        let total_contributions: i64 = by_year_response
            .iter()
            .map(|by_year| {
                by_year
                    .contributions_collection
                    .contribution_calendar
                    .total_contributions
            })
            .sum();

        Ok(total_contributions)
    }

    fn get_stats(&self) -> Result<Stats, anyhow::Error> {
        let mut next_owned = "";
        let mut next_contrib = "";

        let mut repos: Vec<String> = vec![];
        let mut forks = 0;

        let variables = repos_overview::Variables {
            owned_cursor: None,
            contributed_cursor: None,
        };
        let raw_results = post_graphql_blocking::<ReposOverview, _>(
            &self.client,
            &self.graphql_url(),
            variables,
        )?;

        let name = raw_results
            .data
            .as_ref()
            .and_then(|data| data.viewer.name.clone())
            .unwrap_or("No Name".to_string());

        let contributed_repos = raw_results
            .data
            .as_ref()
            .and_then(|data| data.viewer.repositories_contributed_to.nodes.as_ref())
            .map(|nodes| {
                nodes
                    .iter()
                    .filter(|opt| opt.is_some())
                    .flatten()
                    .collect::<Vec<_>>()
            });

        for repo in contributed_repos.iter().flatten() {
            repos.push(repo.name_with_owner.clone());
            forks += repo.fork_count;
        }

        let owned_repos = raw_results
            .data
            .as_ref()
            .map(|data| &data.viewer.repositories);

        let repos = owned_repos
            .iter()
            .flat_map(|repos| &repos.nodes)
            .flatten()
            .flatten()
            .map(|repo| repo.name_with_owner.clone())
            .collect::<Vec<_>>();

        let languages = owned_repos
            .iter()
            .flat_map(|repos| &repos.nodes)
            .flatten()
            .flatten()
            .filter_map(|nodes| nodes.languages.as_ref())
            .filter_map(|languages| languages.edges.as_ref())
            .flatten()
            .flatten()
            .map(|edge| {
                Language::new(
                    edge.node.name.clone(),
                    edge.size,
                    1,
                    edge.node.color.clone().unwrap_or("#000000".to_string()),
                    0.2,
                )
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|lang| (lang.name().to_string(), lang.clone()))
            .collect::<HashMap<_, _>>();

        let total_contributions = self.total_contributions()?;
        let views = self.views(&repos)?;
        let lines_changed = self.lines_changed(&repos)?;

        Ok(Stats::builder()
            .name(name)
            .total_contributions(total_contributions)
            .views(views)
            .lines_changed(lines_changed)
            .repos(repos)
            .forks(forks)
            .languages(languages)
            .build())
    }

    fn views(&self, repos: &Vec<String>) -> Result<i64, anyhow::Error> {
        let mut views = 0;

        for repo in repos {
            let response = self
                .client
                .get(format!("{}/repos/{}/traffic/views", &self.url, repo))
                .send()?;
            let json = response.json::<ViewTraffic>()?;
            let sum: i64 = json.views().iter().map(|view| view.count()).sum();
            views += sum;
        }
        Ok(views)
    }

    fn lines_changed(&self, repos: &Vec<String>) -> Result<(i64, i64), anyhow::Error> {
        let res = repos
            .iter()
            .map(|repo| -> Result<Vec<ContributorActivity>, reqwest::Error> {
                let response = self
                    .client
                    .get(format!("{}/repos/{}/stats/contributors", &self.url, repo))
                    .send()?;
                let json = response.json::<Vec<ContributorActivity>>();
                json
            })
            .filter_map(Result::ok)
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .fold((0, 0), |acc, activity| {
                (
                    acc.0
                        + activity
                            .weeks()
                            .iter()
                            .map(|week| week.added())
                            .sum::<i64>(),
                    acc.1
                        + activity
                            .weeks()
                            .iter()
                            .map(|week| week.deleted())
                            .sum::<i64>(),
                )
            });

        Ok(res)
    }
}
