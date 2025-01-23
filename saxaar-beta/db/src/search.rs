use crate::traits::FromRow;
use anyhow::Result;
use models::{
    place::{Place, PlaceType},
    voyage::Voyage,
    yew::NodeType,
};
use rusqlite::{Connection, Result as SqlResult, Row};
use std::{rc::Rc, str::FromStr};

// Implement FromRow for our Place type
impl FromRow for Place {
    fn from_row(row: &Row) -> SqlResult<Self> {
        let place_type = PlaceType::from_str(&row.get::<_, String>("place_type")?).unwrap();
        Ok(Place {
            id: row.get("id")?,
            place_type,
            value: row.get("value")?,
            name: row.get("name")?,
            region_value: row.get("region_value")?,
            region_name: row.get("region_name")?,
            broad_region_value: row.get("broad_region_value")?,
            broad_region_name: row.get("broad_region_name")?,
        })
    }
}

// Implement FromRow for our Voyage type
impl FromRow for Voyage {
    fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(Voyage {
            id: row.get("id")?,
            origin_port: row.get("origin_port")?,
            origin_region: row.get("origin_region")?,
            origin_broad_region: row.get("origin_broad_region")?,
            destination_port: row.get("destination_port")?,
            destination_region: row.get("destination_region")?,
            destination_broad_region: row.get("destination_broad_region")?,
            embark_date: row.get("embark_date")?,
            disembark_date: row.get("disembark_date")?,
            slaves_embarked: row.get("slaves_embarked")?,
            slaves_disembarked: row.get("slaves_disembarked")?,
        })
    }
}

// Helper function to query with FromRow trait
fn query_rows<T: FromRow>(
    database_connection: &Connection,
    query: &str,
    params: &[&dyn rusqlite::ToSql],
) -> Result<Vec<T>> {
    let mut statement = database_connection.prepare(query)?;
    let rows = statement.query_map(params, |row| T::from_row(row))?;

    let results: Result<Vec<_>, _> = rows.collect();
    Ok(results?)
}

/// Search for places matching the given criteria
///
/// This function searches the places table based on the provided query
/// and optional constraints from an already selected place.
///
/// # Arguments
/// * `database_connection` - Reference to the database connection
/// * `search_query` - The search term
/// * `node_type` - Type of the search node (Origin/Destination)
/// * `other_place` - Optional selected place to constrain the search
///
/// # Returns
/// * `Result<Vec<Place>>` - Matching places or an error
pub fn search_places(
    database_connection: &Rc<Connection>,
    search_query: &str,
    node_type: &NodeType,
    other_place: Option<&Place>,
) -> Result<Vec<Place>> {
    log::info!(
        "search_places called with query: '{}', node_type: {:?}, other_place: {:?}",
        search_query,
        node_type,
        other_place.map(|p| format!("{}:{}", p.name, p.value))
    );
    // Create the formatted strings
    let like_pattern = if search_query.is_empty() {
        "%".to_string()
    } else {
        format!("%{}%", search_query.trim())
    };

    let (query, params) = match (node_type, other_place) {
        (_, None) => {
            // Basic search without constraints
            (
                "SELECT DISTINCT p.*,
                    CASE 
                        WHEN name LIKE ? THEN 1
                        ELSE 2
                    END as match_rank
                FROM places p
                WHERE name LIKE ?
                ORDER BY match_rank, length(name)
                LIMIT 10",
                vec![&like_pattern as &dyn rusqlite::ToSql, &like_pattern],
            )
        }
        (NodeType::Destination, Some(origin)) => {
            // Search for destinations that have voyages from the selected origin
            (
                "SELECT DISTINCT p.*,
                    CASE 
                        WHEN name LIKE ? THEN 1
                        ELSE 2
                    END as match_rank
                FROM places p
                WHERE name LIKE ?
                AND EXISTS (
                    SELECT 1 FROM voyages v
                    WHERE (
                        v.origin_port = ? 
                        OR v.origin_region = ? 
                        OR v.origin_broad_region = ?
                    )
                    AND (
                        v.destination_port = p.value
                        OR v.destination_region = p.value
                        OR v.destination_broad_region = p.value
                    )
                )
                ORDER BY match_rank, length(name)
                LIMIT 10",
                vec![
                    &like_pattern as &dyn rusqlite::ToSql,
                    &like_pattern,
                    &origin.value,
                    &origin.value,
                    &origin.value,
                ],
            )
        }
        (NodeType::Origin, Some(destination)) => {
            // Search for origins that have voyages to the selected destination
            (
                "SELECT DISTINCT p.*,
                    CASE 
                        WHEN name LIKE ? THEN 1
                        ELSE 2
                    END as match_rank
                FROM places p
                WHERE name LIKE ?
                AND EXISTS (
                    SELECT 1 FROM voyages v
                    WHERE (
                        v.destination_port = ? 
                        OR v.destination_region = ? 
                        OR v.destination_broad_region = ?
                    )
                    AND (
                        v.origin_port = p.value
                        OR v.origin_region = p.value
                        OR v.origin_broad_region = p.value
                    )
                )
                ORDER BY match_rank, length(name)
                LIMIT 10",
                vec![
                    &like_pattern as &dyn rusqlite::ToSql,
                    &like_pattern,
                    &destination.value,
                    &destination.value,
                    &destination.value,
                ],
            )
        }
    };

    let mut statement = database_connection.prepare(query)?;
    let rows = statement.query_map(params.as_slice(), |row| {
        Ok(Place {
            id: row.get("id")?,
            place_type: PlaceType::from_str(&row.get::<_, String>("place_type")?).unwrap(),
            value: row.get("value")?,
            name: row.get("name")?,
            region_value: row.get("region_value")?,
            region_name: row.get("region_name")?,
            broad_region_value: row.get("broad_region_value")?,
            broad_region_name: row.get("broad_region_name")?,
        })
    })?;

    let places: Result<Vec<_>, _> = rows.collect();
    let places = places?;

    // Log the executed query and results
    log::info!("Executed SQL query: {}", query);
    log::info!("Like pattern used: {}", like_pattern);

    // Log additional context based on the query type
    match (node_type, other_place) {
        (NodeType::Destination, Some(origin)) => {
            log::info!(
                "Searching for destinations with origin: {} (value: {})",
                origin.name,
                origin.value
            );
        }
        (NodeType::Origin, Some(destination)) => {
            log::info!(
                "Searching for origins with destination: {} (value: {})",
                destination.name,
                destination.value
            );
        }
        (_, None) => {
            log::info!("Performing unconstrained search");
        }
    }

    log::info!("Found {} matching places", places.len());

    // Log each found place for debugging
    for place in places.iter() {
        log::info!(
            "Found place: {} (value: {}, region: {:?}, broad_region: {:?})",
            place.name,
            place.value,
            place.region_name,
            place.broad_region_name
        );
    }

    Ok(places)
}

/// Retrieve voyages between two places
///
/// Finds all voyages that connect the specified origin and destination,
/// including matches at port, region, or broad region level.
///
/// # Arguments
/// * `database_connection` - Reference to the database connection
/// * `origin` - The origin place
/// * `destination` - The destination place
///
/// # Returns
/// * `Result<Vec<Voyage>>` - Matching voyages or an error
pub fn get_voyages(
    database_connection: &Rc<Connection>,
    origin: &Place,
    destination: &Place,
) -> Result<Vec<Voyage>> {
    let sql_query = "
        SELECT 
            voyages.*,
            origin_places.name as origin_place_name,
            destination_places.name as destination_place_name
        FROM voyages
        LEFT JOIN places as origin_places 
            ON voyages.origin_port = origin_places.value
        LEFT JOIN places as destination_places 
            ON voyages.destination_port = destination_places.value
        WHERE (
            voyages.origin_port = ?1 
            OR voyages.origin_region = ?1 
            OR voyages.origin_broad_region = ?1
        )
        AND (
            voyages.destination_port = ?2 
            OR voyages.destination_region = ?2 
            OR voyages.destination_broad_region = ?2
        )
        ORDER BY 
            CASE 
                WHEN voyages.embark_date IS NOT NULL THEN 0
                ELSE 1
            END,
            voyages.embark_date";

    query_rows::<Voyage>(database_connection, sql_query, &[
        &origin.value,
        &destination.value,
    ])
}

#[cfg(test)]
mod search_tests {
    use super::*;
    use crate::table::*;
    use std::rc::Rc;

    fn setup_test_db() -> Rc<Connection> {
        let connection = Connection::open_in_memory().unwrap();
        let mut connection = Rc::new(connection);
        
        // Wrap the entire initialization in a transaction
        let tx = connection.transaction().unwrap();
    
        // Initialize places table first
        init_places_table(connection.clone()).unwrap();
    
        // Now initialize voyages table 
        init_voyages_table(connection.clone()).unwrap();
    
        // Commit transaction
        tx.commit().unwrap();
    
        connection
    }

    fn find_place_by_name(places: &[Place], name: &str) -> Option<Place> {
        places.iter().find(|p| p.name == name).cloned()
    }

    #[test]
    fn test_delagoa_to_charleston() {
        let db = setup_test_db();

        // First find Delagoa and set it as origin
        let all_results = search_places(&db, "delagoa", &NodeType::Origin, None).unwrap();
        let delagoa = find_place_by_name(&all_results, "Delagoa")
            .expect("Should find Delagoa in initial search");

        // Now search for Charleston as destination
        let results = search_places(&db, "char", &NodeType::Destination, Some(&delagoa)).unwrap();

        assert!(
            results.iter().any(|p| p.name == "Charleston"),
            "Charleston should be a valid destination from Delagoa\nFound places: {:#?}",
            results
        );
    }

    #[test]
    fn test_delagoa_to_montserrat() {
        let db = setup_test_db();

        // First find Delagoa
        let all_results = search_places(&db, "delagoa", &NodeType::Origin, None).unwrap();
        let delagoa = find_place_by_name(&all_results, "Delagoa")
            .expect("Should find Delagoa in initial search");

        // Now search for Montserrat as destination
        let results = search_places(&db, "mont", &NodeType::Destination, Some(&delagoa)).unwrap();

        assert!(
            results.iter().any(|p| p.name == "Montserrat"),
            "Montserrat should be a valid destination from Delagoa\nFound places: {:#?}",
            results
        );
    }

    #[test]
    fn test_charleston_to_delagoa() {
        let db = setup_test_db();

        // First find Charleston
        let all_results = search_places(&db, "charleston", &NodeType::Destination, None).unwrap();
        let charleston = find_place_by_name(&all_results, "Charleston")
            .expect("Should find Charleston in initial search");

        // Now search for Delagoa as origin
        let results = search_places(&db, "dela", &NodeType::Origin, Some(&charleston)).unwrap();

        assert!(
            results.iter().any(|p| p.name == "Delagoa"),
            "Delagoa should be a valid origin for Charleston\nFound places: {:#?}",
            results
        );
    }

    #[test]
    fn test_montserrat_to_delagoa() {
        let db = setup_test_db();

        // First find Montserrat
        let all_results = search_places(&db, "montserrat", &NodeType::Destination, None).unwrap();
        let montserrat = find_place_by_name(&all_results, "Montserrat")
            .expect("Should find Montserrat in initial search");

        // Now search for Delagoa as origin
        let results = search_places(&db, "dela", &NodeType::Origin, Some(&montserrat)).unwrap();

        assert!(
            results.iter().any(|p| p.name == "Delagoa"),
            "Delagoa should be a valid origin for Montserrat\nFound places: {:#?}",
            results
        );
    }

    #[test]
    fn test_voyages_exist() {
        let db = setup_test_db();

        // Direct query to check if voyages exist with these connections
        let sql = "
            SELECT COUNT(*) 
            FROM voyages 
            WHERE (origin_port = 60206 AND (destination_port = 21302 OR destination_port = 33799))
               OR (origin_port = 60206 AND (destination_region = 21300 OR destination_region = 33700))
               OR (origin_port = 60206 AND (destination_broad_region = 20000 OR destination_broad_region = 30000))
        ";

        let count: i32 = db.query_row(sql, [], |row| row.get(0)).unwrap();

        assert!(
            count > 0,
            "Should find voyages connecting Delagoa with Charleston/Montserrat in the database"
        );
    }
}

#[cfg(test)]
mod db_initialization_tests {
    use rusqlite::params;

    use super::*;
    use crate::table::init_db;
    use std::rc::Rc;

    // fn setup_test_db() -> Rc<Connection> {
    //     let connection = Connection::open_in_memory().unwrap();
    //     let connection = Rc::new(connection);
    //     init_db(connection.clone()).unwrap();
    //     connection
    // }

    fn setup_test_db() -> Rc<Connection> {
        let connection = Connection::open_in_memory().unwrap();
        let connection = Rc::new(connection);
        
        // Wrap the entire initialization in a transaction
        let tx = connection.transaction().unwrap();
    
        // Initialize places table first
        init_places_table(connection.clone()).unwrap();
    
        // Now initialize voyages table 
        init_voyages_table(connection.clone()).unwrap();
    
        // Commit transaction
        tx.commit().unwrap();
    
        connection
    }

    #[test]
    fn test_init_db_creates_tables() {
        let db = setup_test_db();

        // Verify tables exist
        let table_count: i32 = db.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND (name='places' OR name='voyages')",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(
            table_count, 2,
            "Should create both places and voyages tables"
        );
    }

    #[test]
    fn test_delagoa_exists_in_places() {
        let db = setup_test_db();

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM places WHERE value = 60206",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 1, "Delagoa should be loaded into places table");
    }

    #[test]
    fn test_charleston_exists_in_places() {
        let db = setup_test_db();

        let count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM places WHERE value = 21302",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 1, "Charleston should be loaded into places table");
    }

    #[test]
    fn test_voyage_data_loading() {
        let db = setup_test_db();

        // First verify we have any voyages at all
        let total_voyages: i32 = db
            .query_row("SELECT COUNT(*) FROM voyages", [], |row| row.get(0))
            .unwrap();

        assert!(
            total_voyages > 0,
            "Should have loaded some voyages into database"
        );

        // Log the first few voyages for inspection
        let mut stmt = db
            .prepare("SELECT id, origin_port, destination_port FROM voyages LIMIT 5")
            .unwrap();

        let voyages: Vec<(i64, Option<i32>, Option<i32>)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        println!("First few voyages: {:#?}", voyages);
    }

    #[test]
    fn test_specific_voyage_pairs() {
        let db = setup_test_db();

        // List some known voyage pairs from random_sample.csv
        let test_cases = vec![
            // format: (origin_port, destination_port, expected_count)
            (60206, 21302, "Delagoa to Charleston"),
            (60206, 33799, "Delagoa to Montserrat"),
        ];

        for (origin, dest, description) in test_cases {
            let count: i32 = db
                .query_row(
                    "SELECT COUNT(*) FROM voyages 
                WHERE origin_port = ? AND destination_port = ?",
                    [origin, dest],
                    |row| row.get(0),
                )
                .unwrap();

            println!("Testing voyage {}: found {} records", description, count);

            // Also check region and broad region connections
            let region_count: i32 = db.query_row(
                "SELECT COUNT(*) FROM voyages
                WHERE (origin_port = ? OR origin_region = ? OR origin_broad_region = ?)
                AND (destination_port = ? OR destination_region = ? OR destination_broad_region = ?)",
                [origin, origin, origin, dest, dest, dest],
                |row| row.get(0)
            ).unwrap();

            println!(
                "Including regions for {}: found {} records",
                description, region_count
            );
        }
    }

    #[test]
    fn test_voyage_loading_from_csv() {
        use csv::Reader;
        use rusqlite::Connection;
        use std::rc::Rc;

        // Set up a test database
        let db = Rc::new(Connection::open_in_memory().unwrap());

        // First create both tables
        db.execute(
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
        )
        .unwrap();

        db.execute(
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
            FOREIGN KEY(origin_port) REFERENCES places(value),
            FOREIGN KEY(origin_region) REFERENCES places(value),
            FOREIGN KEY(origin_broad_region) REFERENCES places(value),
            FOREIGN KEY(destination_port) REFERENCES places(value),
            FOREIGN KEY(destination_region) REFERENCES places(value),
            FOREIGN KEY(destination_broad_region) REFERENCES places(value)
        )",
            [],
        )
        .unwrap();

        // Read directly from random_sample.csv
        let voyages_str = include_str!("../../fixtures/random_sample.csv");
        let mut reader = Reader::from_reader(voyages_str.as_bytes());

        // Count records and attempted inserts
        let mut total_records = 0;
        let mut insert_attempts = 0;
        let mut successful_inserts = 0;
        let mut parse_errors = 0;
        let mut insert_errors = 0;

        for result in reader.deserialize() {
            total_records += 1;

            match result {
                Ok(voyage) => {
                    insert_attempts += 1;
                    let voyage: Voyage = voyage;

                    match db.execute(
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
                    ) {
                        Ok(rows) => successful_inserts += rows as i32,
                        Err(e) => {
                            insert_errors += 1;
                            println!("Insert error for voyage {}: {}", voyage.id, e);
                        }
                    }
                }
                Err(e) => {
                    parse_errors += 1;
                    println!("Parse error: {}", e);
                }
            }
        }

        println!("Voyage loading statistics:");
        println!("Total records: {}", total_records);
        println!("Parse errors: {}", parse_errors);
        println!("Insert attempts: {}", insert_attempts);
        println!("Successful inserts: {}", successful_inserts);
        println!("Insert errors: {}", insert_errors);

        // Verify results
        let count: i32 = db
            .query_row("SELECT COUNT(*) FROM voyages", [], |row| row.get(0))
            .unwrap();

        assert!(count > 0, "Should have loaded voyages into database");
    }

    #[test]
    fn test_places_loading() {
        let db = setup_test_db();

        // Check how a specific place is stored
        let mut stmt = db
            .prepare(
                "
        SELECT place_type, value, name, region_value, region_name, 
               broad_region_value, broad_region_name 
        FROM places 
        WHERE value = 60206
    ",
            )
            .unwrap();

        let row = stmt
            .query_row([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i32>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<i32>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<i32>>(5)?,
                    row.get::<_, Option<String>>(6)?,
                ))
            })
            .unwrap();

        println!("Delagoa record:");
        println!("- place_type: {}", row.0);
        println!("- value: {}", row.1);
        println!("- name: {}", row.2);
        println!("- region_value: {:?}", row.3);
        println!("- region_name: {:?}", row.4);
        println!("- broad_region_value: {:?}", row.5);
        println!("- broad_region_name: {:?}", row.6);

        // Check if we have stored region and broad region values as separate place entries
        let region_count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM places WHERE place_type = 'SpecificRegion'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        let broad_region_count: i32 = db
            .query_row(
                "SELECT COUNT(*) FROM places WHERE place_type = 'BroadRegion'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        println!("\nPlace type counts:");
        println!("Regions: {}", region_count);
        println!("Broad regions: {}", broad_region_count);
    }

    #[test]
fn test_place_insertion_details() {
    let db = setup_test_db();
    
    // Check total number of places
    let total_places: i32 = db.query_row(
        "SELECT COUNT(*) FROM places",
        [],
        |row| row.get(0)
    ).unwrap();
    
    println!("Total places in database: {}", total_places);
    
    // Sample some specific places
    let mut stmt = db.prepare("
        SELECT place_type, value, name, region_value, region_name, 
               broad_region_value, broad_region_name 
        FROM places 
        LIMIT 5
    ").unwrap();
    
    let places: Vec<(String, i32, String)> = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, String>(2)?,
        ))
    }).unwrap().map(|r| r.unwrap()).collect();
    
    println!("Sample places:");
    for (place_type, value, name) in places {
        println!("- {} ({}): {}", name, value, place_type);
    }
}

#[test]
fn test_voyage_insertion_process() {
    let db = setup_test_db();
    
    // First check what places we have that could be used as foreign keys
    let place_values: Vec<i32> = db.prepare("SELECT value FROM places")
        .unwrap()
        .query_map([], |row| row.get::<_, i32>(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    
    println!("Available place values: {:?}", place_values);
    
    // Try to insert a test voyage using known place values
    if let Some(test_place) = place_values.first() {
        let result = db.execute(
            "INSERT INTO voyages (
                id, origin_port, destination_port
            ) VALUES (?, ?, ?)",
            params![1, test_place, test_place],
        );
        
        match result {
            Ok(_) => println!("Successfully inserted test voyage"),
            Err(e) => println!("Failed to insert test voyage: {}", e),
        }
    }
}
}
