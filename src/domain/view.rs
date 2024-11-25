use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ViewTraffic {
    count: i64,
    uniques: i64,
    views: Vec<View>,
}

impl ViewTraffic {
    pub fn views(&self) -> &Vec<View> {
        self.views.as_ref()
    }
}

#[derive(Serialize, Deserialize)]
pub struct View {
    timestamp: DateTime<Utc>,
    count: i64,
    uniques: i64,
}

impl View {
    pub fn count(&self) -> i64 {
        self.count
    }
}
