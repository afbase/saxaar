use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TemporalPattern {
    pub year: i32,
    pub month: Option<i32>,
    pub voyage_count: i64,
    pub total_embarked: i64,
    pub total_disembarked: i64,
}
