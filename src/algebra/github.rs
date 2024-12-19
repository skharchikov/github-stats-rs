use crate::domain::Stats;

use anyhow::Result;

pub trait GithubExt {
    type CalendarWeek;

    async fn total_contributions(&self) -> Result<i64>;
    async fn get_stats(&self) -> Result<Stats>;
    async fn views(&self, repos: &[String]) -> Result<i64>;
    async fn lines_changed(&self, repos: &[String]) -> Result<(i64, i64)>;
    async fn contribution_calendar(&self) -> Result<Vec<Self::CalendarWeek>>;
}
