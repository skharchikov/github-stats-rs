use std::collections::HashMap;

pub struct Stats {
    name: Option<String>,
    stargazers: Option<i32>,
    forks: Option<i32>,
    total_contributions: Option<i32>,
    languages: Option<HashMap<String, String>>,
    repos: Option<Vec<String>>,
    lines_changed: Option<(i32, i32)>,
    views: Option<i32>,
}

impl ToString for Stats {
    fn to_string(&self) -> String {
        format!(
            r#"Name: {}
Stargazers: {}
Forks: {}
Total contributions: {}"#,
            self.name.clone().unwrap_or("".to_string()),
            self.stargazers.unwrap_or_default(),
            self.forks.unwrap_or_default(),
            self.total_contributions.unwrap_or_default(),
        )
    }
}
