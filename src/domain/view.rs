use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ViewTraffic {
    count: i32,
    uniques: i32,
    views: Vec<View>,
}

#[derive(Serialize, Deserialize)]
pub struct View {
    timestamp: DateTime<Utc>,
    count: i32,
    uniques: i32,
}
