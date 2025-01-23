use crate::error::DatabaseError;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Place {
    pub id: i64,
    pub place_type: PlaceType,
    pub value: i32,
    pub name: String,
    pub region_value: Option<i32>,
    pub region_name: Option<String>,
    pub broad_region_value: Option<i32>,
    pub broad_region_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlaceType {
    Port,
    SpecificRegion,
    BroadRegion,
}

impl FromStr for PlaceType {
    type Err = DatabaseError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "Port" => Ok(PlaceType::Port),
            "SpecificRegion" => Ok(PlaceType::SpecificRegion),
            "BroadRegion" => Ok(PlaceType::BroadRegion),
            invalid => Err(DatabaseError::InvalidPlaceType(invalid.to_string())),
        }
    }
}

impl Display for PlaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PlaceType::Port => write!(f, "Port"),
            PlaceType::SpecificRegion => write!(f, "SpecificRegion"),
            PlaceType::BroadRegion => write!(f, "BroadRegion"),
        }
    }
}
