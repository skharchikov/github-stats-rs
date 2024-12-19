use derive_builder::Builder;

use super::contribution_calendar::ContributionCalendarUserContributionsCollectionContributionCalendarWeeks;

pub type CalendarWeek = ContributionCalendarUserContributionsCollectionContributionCalendarWeeks;

#[derive(Builder)]
pub struct Stats {
    name: String,
    stargazers: i64,
    forks: i64,
    total_contributions: i64,
    /// A Vec of languages with their name as key and the Language struct as value
    /// Sorted by the size of the language
    languages: Vec<(String, Language)>,
    repos: Vec<String>,
    lines_changed: (i64, i64),
    views: i64,
    contribution_calendar: Vec<CalendarWeek>,
}

impl Stats {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stargazers(&self) -> i64 {
        self.stargazers
    }

    pub fn forks(&self) -> i64 {
        self.forks
    }

    pub fn total_contributions(&self) -> i64 {
        self.total_contributions
    }

    pub fn languages(&self) -> &Vec<(String, Language)> {
        &self.languages
    }

    pub fn repos(&self) -> &Vec<String> {
        self.repos.as_ref()
    }

    pub fn lines_changed(&self) -> (i64, i64) {
        self.lines_changed
    }

    pub fn views(&self) -> i64 {
        self.views
    }

    pub fn contribution_calendar(&self) -> &[CalendarWeek] {
        &self.contribution_calendar
    }
}

#[derive(Debug, Clone)]
pub struct Language {
    name: String,
    size: i64,
    occurrences: i64,
    color: String,
    proportion: f64,
}

impl Language {
    pub fn new(name: String, size: i64, occurrences: i64, color: String, proportion: f64) -> Self {
        Self {
            name,
            size,
            occurrences,
            color,
            proportion,
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn proportion(&self) -> f64 {
        self.proportion
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_size(&mut self, size: i64) {
        self.size = size;
    }

    pub fn set_occurrences(&mut self, occurrences: i64) {
        self.occurrences = occurrences;
    }

    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn occurrences(&self) -> i64 {
        self.occurrences
    }

    pub fn set_proportion(&mut self, total_size: i64) {
        let proportion = 100f64 * self.size as f64 / total_size as f64;
        self.proportion = proportion;
    }
}
