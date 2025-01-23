use crate::place::{Place, PlaceType};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;
pub const PORTS: &str = include_str!("../../fixtures/geography.csv");
//"Broad Region Value","Broad Region","Specific Region Value","Specific Region (country or colony)","Place Value","Place (port or location)"
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[serde(rename = "Broad Region Value")]
    pub broad_region_value: i32,
    #[serde(rename = "Broad Region")]
    pub broad_region: String,
    #[serde(rename = "Specific Region Value")]
    pub specific_region_value: i32,
    #[serde(rename = "Specific Region (country or colony)")]
    pub specific_region: String,
    #[serde(rename = "Place Value")]
    pub value: i32,
    #[serde(rename = "Place (port or location)")]
    pub name: String,
}

impl From<Port> for Place {
    fn from(port: Port) -> Self {
        Place {
            id: 0, // This will be set by the database
            place_type: PlaceType::Port,
            value: port.value,
            name: port.name,
            region_value: Some(port.specific_region_value),
            region_name: Some(port.specific_region),
            broad_region_value: Some(port.broad_region_value),
            broad_region_name: Some(port.broad_region),
        }
    }
}

#[derive(Debug, Error)]
pub enum PortConversionError {
    #[error("Invalid place type: expected Port, got {0:?}")]
    InvalidPlaceType(PlaceType),
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}

impl TryFrom<Place> for Port {
    type Error = PortConversionError;

    fn try_from(place: Place) -> Result<Self, Self::Error> {
        if place.place_type != PlaceType::Port {
            return Err(PortConversionError::InvalidPlaceType(place.place_type));
        }

        Ok(Port {
            broad_region_value: place
                .broad_region_value
                .ok_or(PortConversionError::MissingField("broad_region_value"))?,
            broad_region: place
                .broad_region_name
                .ok_or(PortConversionError::MissingField("broad_region"))?,
            specific_region_value: place
                .region_value
                .ok_or(PortConversionError::MissingField("specific_region_value"))?,
            specific_region: place
                .region_name
                .ok_or(PortConversionError::MissingField("specific_region"))?,
            value: place.value,
            name: place.name,
        })
    }
}
