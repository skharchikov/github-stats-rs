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
        let mut next_owned = None;
        let mut next_contrib = None;

        let mut name = None;
        let mut repos: Vec<String> = vec![];
        let mut forks = 0;
        let mut stargazers = 0;
        let mut languages: HashMap<String, Language> = HashMap::new();

        loop {
            let variables = repos_overview::Variables {
                owned_cursor: next_owned,
                contributed_cursor: next_contrib,
            };
            let raw_results = post_graphql_blocking::<ReposOverview, _>(
                &self.client,
                &self.graphql_url(),
                variables,
            )?;

            name = name.or(raw_results
                .data
                .as_ref()
                .and_then(|data| data.viewer.name.clone()));

            let mut languages_contributed = languages;

            if self.configuration.exclude_forked_repos() {
                // do nothing
            } else {
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
                    stargazers += repo.stargazers.total_count;
                }

                languages_contributed = contributed_repos
                    .iter()
                    .flatten()
                    .filter_map(|repo| repo.languages.as_ref())
                    .filter_map(|languages| languages.edges.as_ref())
                    .flatten()
                    .flatten()
                    .map(|edge| {
                        Language::new(
                            edge.node.name.clone(),
                            edge.size,
                            1,
                            edge.node.color.clone().unwrap_or("#000000".to_string()),
                            0.0,
                        )
                    })
                    .collect::<Vec<_>>()
                    .iter()
                    .fold(HashMap::new(), |mut acc, lang| {
                        acc.entry(lang.name().to_string())
                            .and_modify(|e: &mut Language| {
                                let new_zise: i64 = e.size() + lang.size();
                                let new_occurences: i64 = e.occurrences() + lang.occurrences();
                                e.set_occurrences(new_occurences);
                                e.set_size(new_zise);
                            })
                            .or_insert(lang.clone());
                        acc
                    });
            }

            let owned_repos = raw_results
                .data
                .as_ref()
                .map(|data| &data.viewer.repositories);

            for repo in owned_repos
                .iter()
                .flat_map(|repos| &repos.nodes)
                .flatten()
                .flatten()
            {
                repos.push(repo.name_with_owner.clone());
                forks += repo.fork_count;
                stargazers += repo.stargazer_count;
            }

            languages = owned_repos
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
                        0.0,
                    )
                })
                .collect::<Vec<_>>()
                .iter()
                .fold(languages_contributed, |mut acc, lang| {
                    acc.entry(lang.name().to_string())
                        .and_modify(|e: &mut Language| {
                            let new_zise: i64 = e.size() + lang.size();
                            let new_occurences: i64 = e.occurrences() + lang.occurrences();
                            e.set_occurrences(new_occurences);
                            e.set_size(new_zise);
                        })
                        .or_insert(lang.clone());
                    acc
                });
            let total_size = languages.values().map(|lang| lang.size()).sum::<i64>();
            languages.iter_mut().for_each(|(_, lang)| {
                lang.set_proportion(100f64 * lang.size() as f64 / total_size as f64);
            });

            let has_next_owned = owned_repos
                .as_ref()
                .map(|repos| repos.page_info.has_next_page)
                .unwrap_or_default();

            let has_next_contrib = !self.configuration.exclude_forked_repos()
                && raw_results
                    .data
                    .as_ref()
                    .map(|data| {
                        data.viewer
                            .repositories_contributed_to
                            .page_info
                            .has_next_page
                    })
                    .unwrap_or_default();

            if has_next_owned || has_next_contrib {
                next_owned = owned_repos
                    .as_ref()
                    .and_then(|repos| repos.page_info.end_cursor.as_ref().cloned());
                next_contrib = raw_results.data.as_ref().and_then(|data| {
                    data.viewer
                        .repositories_contributed_to
                        .page_info
                        .end_cursor
                        .as_ref()
                        .cloned()
                });
            } else {
                break;
            }
        }
        let total_contributions = self.total_contributions()?;
        let views = self.views(&repos)?;
        let lines_changed = self.lines_changed(&repos)?;

        Ok(Stats::builder()
            .name(name.unwrap_or_default())
            .total_contributions(total_contributions)
            .views(views)
            .lines_changed(lines_changed)
            .repos(repos)
            .forks(forks)
            .stargazers(stargazers)
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
