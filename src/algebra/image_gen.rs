use crate::domain::Stats;

pub trait ImageGenExt {
    fn generate_overview(&self, stats: &Stats) -> Result<(), anyhow::Error>;
    fn generate_languages(&self, stats: &Stats) -> Result<(), anyhow::Error>;
    fn generate_contributions_grid(&self, stats: &Stats) -> Result<(), anyhow::Error>;
}
