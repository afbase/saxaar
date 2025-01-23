use crate::place::Place;
use serde::{Deserialize, Serialize};
/// Represents the result of a voyage route analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct RouteAnalysis {
    pub origin_place: Place,
    pub destination_place: Place,
    pub total_voyages: i64,
    pub total_embarked: i64,
    pub total_disembarked: i64,
    pub average_journey_days: f64,
    pub mortality_rate: f64,
}
