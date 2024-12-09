use crate::domain::Stats;

use anyhow::Result;

pub trait GithubExt {
    type CalendarWeek;

    fn total_contributions(&self) -> Result<i64>;
    fn get_stats(&self) -> Result<Stats>;
    fn views(&self, repos: &[String]) -> Result<i64>;
    fn lines_changed(&self, repos: &[String]) -> Result<(i64, i64)>;
    fn contribution_calendar(&self) -> Result<Vec<Self::CalendarWeek>>;
}
