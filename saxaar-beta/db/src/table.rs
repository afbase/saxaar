use anyhow::{Error, Result};
use csv::Reader;
use models::error::DatabaseError;
use models::place::PlaceType;
use models::{
    place::Place,
    port::{PORTS, Port},
    voyage::{VOYAGES, Voyage},
};
use rusqlite::{Connection, Transaction, params};
use std::rc::Rc;

pub fn init_places_table(database_connection: Rc<Connection>) -> Result<()> {
    database_connection.execute(
        "CREATE TABLE IF NOT EXISTS places (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            place_type TEXT NOT NULL,
            value INTEGER NOT NULL,
            name TEXT NOT NULL,
            region_value INTEGER,
            region_name TEXT,
            broad_region_value INTEGER,
            broad_region_name TEXT,
            UNIQUE(value, place_type)
        )",
        [],
    )?;

    // Start a transaction for all inserts
    database_connection.execute("BEGIN TRANSACTION", [])?;

    let mut csv_reader = Reader::from_reader(PORTS.as_bytes());
    
    // First pass: Create broad regions
    for result in csv_reader.deserialize::<Port>() {
        if let Ok(port) = result {
            let broad_value = port.broad_region_value;
            let broad_name = port.broad_region;
            database_connection.execute(
                "INSERT OR IGNORE INTO places (
                    place_type,
                    value,
                    name,
                    region_value,
                    region_name,
                    broad_region_value,
                    broad_region_name
                ) VALUES (?1, ?2, ?3, NULL, NULL, NULL, NULL)",
                params![
                    PlaceType::BroadRegion.to_string(),
                    broad_value,
                    broad_name,
                ],
            )?;
        }
    }

    // Second pass: Create specific regions
    let mut csv_reader = Reader::from_reader(PORTS.as_bytes());
    for result in csv_reader.deserialize::<Port>() {
        if let Ok(port) = result {
            let region_value = port.specific_region_value;
            let region_name = port.specific_region;
            database_connection.execute(
                "INSERT OR IGNORE INTO places (
                    place_type,
                    value,
                    name,
                    region_value,
                    region_name,
                    broad_region_value,
                    broad_region_name
                ) VALUES (?1, ?2, ?3, NULL, NULL, ?4, ?5)",
                params![
                    PlaceType::SpecificRegion.to_string(),
                    region_value,
                    region_name,
                    port.broad_region_value,
                    port.broad_region,
                ],
            )?;
        }
    }

    // Final pass: Create ports
    let mut csv_reader = Reader::from_reader(PORTS.as_bytes());
    for result in csv_reader.deserialize::<Port>() {
        match result {
            Ok(port) => {
                let place: Place = port.into();
                database_connection.execute(
                    "INSERT OR IGNORE INTO places (
                        place_type,
                        value,
                        name,
                        region_value,
                        region_name,
                        broad_region_value,
                        broad_region_name
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![
                        place.place_type.to_string(),
                        place.value,
                        place.name,
                        place.region_value,
                        place.region_name,
                        place.broad_region_value,
                        place.broad_region_name,
                    ],
                )?;
            }
            Err(error) => {
                database_connection.execute("ROLLBACK", [])?;
                return Err(Error::new(DatabaseError::CsvDeserialize(error)));
            }
        }
    }

    // Commit the transaction
    database_connection.execute("COMMIT", [])?;

    Ok(())
}

pub fn init_voyages_table(database_connection: Rc<Connection>) -> Result<()> {
    database_connection.execute(
        "CREATE TABLE IF NOT EXISTS voyages (
            id INTEGER PRIMARY KEY,
            origin_port INTEGER,
            origin_region INTEGER,
            origin_broad_region INTEGER,
            destination_port INTEGER,
            destination_region INTEGER,
            destination_broad_region INTEGER,
            embark_date TEXT,
            disembark_date TEXT,
            slaves_embarked INTEGER,
            slaves_disembarked INTEGER,
            FOREIGN KEY(origin_port) REFERENCES places(value) DEFERRABLE INITIALLY DEFERRED,
            FOREIGN KEY(origin_region) REFERENCES places(value) DEFERRABLE INITIALLY DEFERRED,
            FOREIGN KEY(origin_broad_region) REFERENCES places(value) DEFERRABLE INITIALLY DEFERRED,
            FOREIGN KEY(destination_port) REFERENCES places(value) DEFERRABLE INITIALLY DEFERRED,
            FOREIGN KEY(destination_region) REFERENCES places(value) DEFERRABLE INITIALLY DEFERRED,
            FOREIGN KEY(destination_broad_region) REFERENCES places(value) DEFERRABLE INITIALLY DEFERRED
        )",
        [],
    )?;

    let mut csv_reader = Reader::from_reader(VOYAGES.as_bytes());
    
    // Start a transaction for all inserts
    database_connection.execute("BEGIN TRANSACTION", [])?;
        
    for result in csv_reader.deserialize::<Voyage>() {
        match result {
            Ok(voyage) => {
                database_connection.execute(
                    "INSERT OR IGNORE INTO voyages (
                        id,
                        origin_port,
                        origin_region,
                        origin_broad_region,
                        destination_port,
                        destination_region,
                        destination_broad_region,
                        embark_date,
                        disembark_date,
                        slaves_embarked,
                        slaves_disembarked
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                    params![
                        voyage.id,
                        voyage.origin_port,
                        voyage.origin_region,
                        voyage.origin_broad_region,
                        voyage.destination_port,
                        voyage.destination_region,
                        voyage.destination_broad_region,
                        voyage.embark_date,
                        voyage.disembark_date,
                        voyage.slaves_embarked,
                        voyage.slaves_disembarked,
                    ],
                )?;
            }
            Err(e) => {
                log::warn!("Failed to deserialize voyage row: {}", e);
                database_connection.execute("ROLLBACK", [])?;
                continue;
            }
        }
    }
        
    // Commit the transaction
    database_connection.execute("COMMIT", [])?;

    Ok(())
}

pub fn init_db(database_connection: Rc<Connection>) -> Result<()> {
    init_places_table(database_connection.clone())?;
    init_voyages_table(database_connection.clone())?;
    Ok(())
}