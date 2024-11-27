use crate::domain::Stats;

pub trait GithubExt {
    type ContributedDays;

    fn total_contributions(&self) -> Result<i64, anyhow::Error>;
    fn get_stats(&self) -> Result<Stats, anyhow::Error>;
    fn views(&self, repos: &Vec<String>) -> Result<i64, anyhow::Error>;
    fn lines_changed(&self, repos: &Vec<String>) -> Result<(i64, i64), anyhow::Error>;
    fn contribution_calendar(&self) -> anyhow::Result<Vec<Self::ContributedDays>>;
}
