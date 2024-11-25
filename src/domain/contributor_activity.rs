use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContributorActivity {
    author: Author,
    weeks: Vec<Week>,
}

impl ContributorActivity {
    pub fn weeks(&self) -> &Vec<Week> {
        self.weeks.as_ref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Week {
    #[serde(rename = "a")]
    added: i64,
    #[serde(rename = "d")]
    deleted: i64,
}

impl Week {
    pub fn deleted(&self) -> i64 {
        self.deleted
    }

    pub fn added(&self) -> i64 {
        self.added
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    login: String,
}

impl Author {
    pub fn login(&self) -> &str {
        &self.login
    }
}
