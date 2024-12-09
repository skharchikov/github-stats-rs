use crate::domain::Stats;

use anyhow::Result;

pub trait ImageGenExt {
    fn generate_overview(&self, stats: &Stats) -> Result<()>;
    fn generate_languages(&self, stats: &Stats) -> Result<()>;
    fn generate_contributions_grid(&self, stats: &Stats) -> Result<()>;
}
