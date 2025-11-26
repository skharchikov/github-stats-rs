use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use graphql_client::reqwest::post_graphql;
use reqwest::Client;
use tokio::task::JoinSet;

use crate::{
    algebra::GithubExt,
    domain::{
        contribution_calendar::{
            self, ContributionCalendarUserContributionsCollectionContributionCalendarWeeks,
        },
        contribution_years, contributions_by_year, repos_overview, ContributionCalendar,
        ContributionYears, ContributionsByYear, ContributorActivity, Language, ReposOverview,
        Stats, StatsBuilder, ViewTraffic,
    },
};

use super::Configuration;

#[derive(Debug)]
pub struct Github {
    configuration: Configuration,
    client: Client,
}

impl Github {
    pub fn new(configuration: Configuration, client: Client) -> Self {
        Self {
            configuration,
            client,
        }
    }

    pub fn graphql_url(&self) -> String {
        format!("{}/graphql", self.configuration.github_url())
    }
}

impl GithubExt for Github {
    type CalendarWeek = ContributionCalendarUserContributionsCollectionContributionCalendarWeeks;

    #[tracing::instrument]
    async fn total_contributions(&self) -> Result<i64, anyhow::Error> {
        let variables = contribution_years::Variables {};

        let contribution_years_response =
            post_graphql::<ContributionYears, _>(&self.client, &self.graphql_url(), variables)
                .await?;

        let years = contribution_years_response
            .data
            .map(|data| data.viewer.contributions_collection.contribution_years);

        let variables = years
            .map(|years| {
                years
                    .iter()
                    .filter_map(|year| {
                        let year_i32 = *year as i32;

                        let beggining_of_the_year: Option<DateTime<Utc>> =
                            NaiveDate::from_ymd_opt(year_i32, 1, 1)
                                .and_then(|date| date.and_hms_opt(0, 0, 0))
                                .map(|naive_date_time| {
                                    TimeZone::from_utc_datetime(&Utc, &naive_date_time)
                                });

                        let beggining_of_the_next_year: Option<DateTime<Utc>> =
                            NaiveDate::from_ymd_opt(year_i32 + 1, 1, 1)
                                .and_then(|date| date.and_hms_opt(0, 0, 0))
                                .map(|naive_date_time| {
                                    TimeZone::from_utc_datetime(&Utc, &naive_date_time)
                                });

                        beggining_of_the_year
                            .zip(beggining_of_the_next_year)
                            .map(|(start, end)| contributions_by_year::Variables {
                                from: start.to_rfc3339(),
                                to: end.to_rfc3339(),
                            })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mut result = vec![];
        // why I can't use JoinSet here?

        // I had an error when I tried to use JoinSet here, so I had to use a for loop
        // temporary v
        for variables in variables {
            let response = post_graphql::<ContributionsByYear, _>(
                &self.client,
                &self.graphql_url(),
                variables,
            )
            .await;
            result.push(response);
        }

        let by_year_response = result
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(|response| response.data.map(|data| data.viewer))
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

    #[tracing::instrument]
    async fn get_stats(&self) -> Result<Stats> {
        let mut next_owned = None;
        let mut next_contrib = None;

        let mut name = None;
        let mut repos: Vec<String> = vec![];
        let mut forks = 0;
        let mut stargazers = 0;
        let mut languages_map: HashMap<String, Language> = HashMap::new();

        loop {
            let variables = repos_overview::Variables {
                owned_cursor: next_owned,
                contributed_cursor: next_contrib,
            };
            let raw_results =
                post_graphql::<ReposOverview, _>(&self.client, &self.graphql_url(), variables)
                    .await?;

            name = name.or(raw_results
                .data
                .as_ref()
                .and_then(|data| data.viewer.name.clone()));

            let mut languages_contributed = languages_map;

            if self.configuration.exclude_forked_repos() {
                // in this case we only fetch owned repos
            } else {
                // in this case we fetch both owned and contributed repos
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

            languages_map = owned_repos
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

            let total_size = languages_map.values().map(|lang| lang.size()).sum::<i64>();
            languages_map.iter_mut().for_each(|(_, lang)| {
                lang.set_proportion(total_size);
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
        // sort languages by size and take top N languages as defined in configuration
        let mut languages = languages_map.into_iter().collect::<Vec<_>>();
        languages.sort_by(|a, b| b.1.size().cmp(&a.1.size()));
        languages = languages
            .into_iter()
            .take(self.configuration.languages_limit())
            .collect();
        let total_contributions = self.total_contributions();
        let views = self.views(&repos);
        let lines_changed = self.lines_changed(&repos);
        let calendar = self.contribution_calendar();

        let (total_contributions, views, lines_changed, calendar) =
            tokio::join!(total_contributions, views, lines_changed, calendar);

        let stats = StatsBuilder::default()
            .name(name.unwrap_or_default())
            .total_contributions(total_contributions?)
            .views(views?)
            .lines_changed(lines_changed?)
            .repos(repos)
            .forks(forks)
            .stargazers(stargazers)
            .languages(languages)
            .contribution_calendar(calendar?)
            .build()?;

        Ok(stats)
    }

    #[tracing::instrument]
    async fn views(&self, repos: &[String]) -> Result<i64> {
        let mut views = 0;

        for repo in repos {
            let response = self
                .client
                .get(format!(
                    "{}/repos/{}/traffic/views",
                    &self.configuration.github_url(),
                    repo
                ))
                .send()
                .await?;
            let json = response.json::<ViewTraffic>().await?;
            let sum: i64 = json.views().iter().map(|view| view.count()).sum();
            views += sum;
        }
        Ok(views)
    }

    #[tracing::instrument]
    async fn lines_changed(&self, repos: &[String]) -> Result<(i64, i64)> {
        tracing::debug!("Starting lines_changed for repos: {:?}", repos);

        let mut tasks = JoinSet::new();
        for repo in repos {
            let repo = repo.clone();
            let client = self.client.clone();
            let url = format!(
                "{}/repos/{}/stats/contributors",
                &self.configuration.github_url(),
                repo
            );
            tracing::debug!("Requesting contributor stats from URL: {}", url);

            let client_clone = client.clone();
            let url_clone = url.clone();
            tasks.spawn(async move {
                let result: anyhow::Result<Vec<ContributorActivity>> = async {
                    let response = client_clone
                        .get(&url_clone)
                        .send()
                        .await
                        .map_err(|e| {
                            tracing::error!("HTTP request failed for repo {}: {:?}", repo, e);
                            anyhow::anyhow!("HTTP request failed for repo {}: {}", repo, e)
                        })?;

                    let status = response.status();
                    let text = response.text().await.map_err(|e| {
                        tracing::error!("Failed to get response text for repo {}: {:?}", repo, e);
                        anyhow::anyhow!("Failed to get response text for repo {}: {}", repo, e)
                    })?;

                    let data = serde_json::from_str::<Vec<ContributorActivity>>(&text)
                        .map_err(|e| {
                            tracing::error!(
                                "Failed to parse JSON for repo {} (status: {}): {:?}\nResponse body: {}",
                                repo,
                                status,
                                e,
                                text
                            );
                            anyhow::anyhow!("Failed to parse JSON for repo {}: {}", repo, e)
                        })?;

                    tracing::debug!(
                        "Successfully fetched contributor stats for repo {}",
                        repo
                    );
                    Ok(data)
                }
                .await;

                result
            });
        }

        let mut all_activities = Vec::new();
        while let Some(res) = tasks.join_next().await {
            match res {
                Ok(Ok(contributors)) => {
                    tracing::debug!("Fetched {} contributors", contributors.len());
                    all_activities.extend(contributors);
                }
                Ok(Err(e)) => {
                    tracing::error!("Task failed with error: {:?}", e);
                }
                Err(e) => {
                    tracing::error!("Join error: {:?}", e);
                }
            }
        }

        let res = all_activities.iter().fold((0, 0), |acc, activity| {
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

        tracing::info!("Total lines added: {}, deleted: {}", res.0, res.1);

        Ok(res)
    }

    #[tracing::instrument]
    async fn contribution_calendar(&self) -> Result<Vec<Self::CalendarWeek>> {
        let variables = contribution_calendar::Variables {
            login: self.configuration.github_actor().to_string(),
        };

        let response =
            post_graphql::<ContributionCalendar, _>(&self.client, &self.graphql_url(), variables)
                .await?;

        let result = response
            .data
            .and_then(|data| {
                data.user
                    .map(|user| user.contributions_collection.contribution_calendar.weeks)
            })
            .unwrap_or_default();

        Ok(result)
    }
}
