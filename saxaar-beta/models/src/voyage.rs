use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub const VOYAGES: &str = include_str!("../../fixtures/random_sample.csv");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Voyage {
    #[serde(rename = "VOYAGEID")]
    pub id: i64,

    // Origin Places - make these all Option<i32> to handle empty/invalid values
    #[serde(rename = "MAJBUYPT", deserialize_with = "deserialize_optional_number")]
    pub origin_port: Option<i32>,
    #[serde(rename = "MAJBYIMP", deserialize_with = "deserialize_optional_number")]
    pub origin_region: Option<i32>,
    #[serde(rename = "MAJBYIMP1", deserialize_with = "deserialize_optional_number")]
    pub origin_broad_region: Option<i32>,

    // Destination Places
    #[serde(rename = "MJSLPTIMP", deserialize_with = "deserialize_optional_number")]
    pub destination_port: Option<i32>,
    #[serde(rename = "MJSELIMP", deserialize_with = "deserialize_optional_number")]
    pub destination_region: Option<i32>,
    #[serde(rename = "MJSELIMP1", deserialize_with = "deserialize_optional_number")]
    pub destination_broad_region: Option<i32>,

    // Dates
    #[serde(rename = "DATELEFTAFR")]
    pub embark_date: Option<String>,
    #[serde(rename = "DATELAND1")]
    pub disembark_date: Option<String>,

    // Numbers
    #[serde(rename = "SLAXIMP", deserialize_with = "deserialize_optional_number")]
    pub slaves_embarked: Option<i32>,
    #[serde(rename = "SLAMIMP", deserialize_with = "deserialize_optional_number")]
    pub slaves_disembarked: Option<i32>,
}

// Add this helper function
fn deserialize_optional_number<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::Number(num) => {
            if let Some(n) = num.as_i64() {
                Ok(Some(n as i32))
            } else {
                Ok(None)
            }
        }
        Value::String(s) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                match s.parse::<i32>() {
                    Ok(n) => Ok(Some(n)),
                    Err(_) => Ok(None),
                }
            }
        }
        _ => Ok(None),
    }
}
