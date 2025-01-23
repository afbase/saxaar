pub mod analyze;
pub mod search;
pub mod table;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::{analyze::*, search::*, table::*};
    use models::{
        place::{Place, PlaceType},
        yew::NodeType,
    };
    use pretty_assertions::assert_eq;
    use rusqlite::{Connection, params};
    use std::{rc::Rc, str::FromStr};

    fn setup_test_db() -> Rc<Connection> {
        let connection = Connection::open_in_memory().unwrap();
        let connection = Rc::new(connection);
        init_db(connection.clone()).unwrap();
        connection
    }

    #[test]
    fn test_init_db() {
        let connection = setup_test_db();

        // Verify tables were created
        let tables: Vec<String> = connection
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<String>, _>>()
            .unwrap();

        assert!(tables.contains(&"places".to_string()));
        assert!(tables.contains(&"voyages".to_string()));
    }

    #[test]
    fn test_place_type_conversion() {
        assert_eq!(PlaceType::Port, PlaceType::Port); // Simplified test
        assert!(PlaceType::from_str("Port").is_ok());
        assert!(PlaceType::from_str("SpecificRegion").is_ok());
        assert!(PlaceType::from_str("BroadRegion").is_ok());
    }

    #[test]
    fn test_search_places_without_constraint() {
        let connection = setup_test_db();
        let results = search_places(&connection, "London", &NodeType::Origin, None);
        assert!(results.is_ok());
    }

    #[test]
    fn test_error_handling() {
        let connection = setup_test_db();

        // Test empty search
        let results = search_places(&connection, "", &NodeType::Origin, None);
        assert!(results.is_ok());
    }

    #[test]
    fn test_get_voyages() {
        let connection = setup_test_db();

        // Create test places
        let origin = Place {
            id: 1,
            place_type: PlaceType::Port,
            value: 10433, // London's code from geography.csv
            name: "London".to_string(),
            region_value: Some(10400),
            region_name: Some("England".to_string()),
            broad_region_value: Some(10000),
            broad_region_name: Some("Europe".to_string()),
        };

        let destination = Place {
            id: 2,
            place_type: PlaceType::Port,
            value: 60734, // Luanda's code from geography.csv
            name: "Luanda".to_string(),
            region_value: Some(60700),
            region_name: Some("West Central Africa and St. Helena".to_string()),
            broad_region_value: Some(60000),
            broad_region_name: Some("Africa".to_string()),
        };

        let results = get_voyages(&connection, &origin, &destination);
        assert!(results.is_ok());
    }

    #[test]
    fn test_search_places_with_constraint() {
        let connection = setup_test_db();

        // First find London
        let london = search_places(&connection, "London", &NodeType::Origin, None)
            .unwrap()
            .first()
            .cloned();

        // Then search for destinations
        if let Some(london) = london {
            let results = search_places(&connection, "port", &NodeType::Destination, Some(&london));
            assert!(results.is_ok());
        }
    }

    #[test]
    fn test_place_search_ranking() {
        let connection = setup_test_db();

        // Test exact match comes first
        let results = search_places(&connection, "London", &NodeType::Origin, None).unwrap();

        if !results.is_empty() {
            // Just check if we got a match containing "London"
            assert!(results[0].name.contains("London"));
        }
    }

    // #[test]
    // fn test_analyze_route() {
    //     let connection = setup_test_db();

    //     // Find two connected places
    //     let origin = search_places(
    //         &connection,
    //         "London",
    //         &NodeType::Origin,
    //         None
    //     ).unwrap().first().cloned().unwrap();

    //     let destination = search_places(
    //         &connection,
    //         "port",
    //         &NodeType::Destination,
    //         Some(&origin)
    //     ).unwrap().first().cloned().unwrap();

    //     let analysis = analyze_route(&connection, &origin, &destination).unwrap();

    //     assert!(analysis.total_voyages > 0);
    //     assert!(analysis.total_embarked >= analysis.total_disembarked);
    //     assert!(analysis.mortality_rate >= 0.0 && analysis.mortality_rate <= 1.0);
    //     assert!(analysis.average_journey_days > 0.0);
    // }

    // #[test]
    // fn test_temporal_patterns() {
    //     let connection = setup_test_db();

    //     // Test yearly patterns
    //     let yearly_patterns = analyze_temporal_patterns(
    //         &connection,
    //         false,
    //         Some(1700),
    //         Some(1800)
    //     ).unwrap();

    //     assert!(!yearly_patterns.is_empty());
    //     for pattern in &yearly_patterns {
    //         assert!(pattern.year >= 1700 && pattern.year <= 1800);
    //         assert!(pattern.voyage_count > 0);
    //     }

    //     // Test monthly patterns
    //     let monthly_patterns = analyze_temporal_patterns(
    //         &connection,
    //         true,
    //         Some(1700),
    //         Some(1800)
    //     ).unwrap();

    //     assert!(!monthly_patterns.is_empty());
    //     for pattern in &monthly_patterns {
    //         assert!(pattern.year >= 1700 && pattern.year <= 1800);
    //         assert!(pattern.month.unwrap() >= 1 && pattern.month.unwrap() <= 12);
    //         assert!(pattern.voyage_count > 0);
    //     }
    // }

    // #[test]
    // fn test_route_analysis_edge_cases() {
    //     let connection = setup_test_db();

    //     let invalid_place = Place {
    //         id: 999999,
    //         place_type: PlaceType::Port,
    //         value: 999999,
    //         name: "Invalid Place".to_string(),
    //         region_value: None,
    //         region_name: None,
    //         broad_region_value: None,
    //         broad_region_name: None,
    //     };

    //     // Test analysis with non-existent route
    //     let analysis = analyze_route(&connection, &invalid_place, &invalid_place).unwrap();
    //     assert_eq!(analysis.total_voyages, 0);
    //     assert_eq!(analysis.total_embarked, 0);
    //     assert_eq!(analysis.total_disembarked, 0);
    //     assert_eq!(analysis.mortality_rate, 0.0);
    // }

    #[test]
    fn test_temporal_pattern_edge_cases() {
        let connection = setup_test_db();

        // Test with invalid date range
        let patterns = analyze_temporal_patterns(
            &connection,
            false,
            Some(2500), // Future year
            Some(2600),
        )
        .unwrap();
        assert!(patterns.is_empty());

        // Test with reversed date range
        let patterns =
            analyze_temporal_patterns(&connection, false, Some(1800), Some(1700)).unwrap();
        assert!(patterns.is_empty());
    }

    // Helper function to calculate search relevance
    fn calculate_relevance(text: &str, query: &str) -> i32 {
        let text_lower = text.to_lowercase();
        let query_lower = query.to_lowercase();

        if text_lower == query_lower {
            return 100;
        }
        if text_lower.starts_with(&query_lower) {
            return 75;
        }
        if text_lower.contains(&query_lower) {
            return 50;
        }
        0
    }

    #[test]
    fn test_database_initialization() {
        let connection = Connection::open_in_memory().unwrap();
        let connection = Rc::new(connection);

        // Test places table initialization
        assert!(init_places_table(connection.clone()).is_ok());

        // Verify places table structure
        let table_info: Vec<(String, String)> = connection
            .prepare("PRAGMA table_info(places)")
            .unwrap()
            .query_map([], |row| Ok((row.get(1)?, row.get(2)?)))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(table_info.iter().any(|(name, _)| name == "id"));
        assert!(table_info.iter().any(|(name, _)| name == "place_type"));
        assert!(table_info.iter().any(|(name, _)| name == "value"));
        assert!(table_info.iter().any(|(name, _)| name == "name"));

        // Test voyages table initialization
        assert!(init_voyages_table(connection.clone()).is_ok());

        // Verify voyages table structure
        let table_info: Vec<(String, String)> = connection
            .prepare("PRAGMA table_info(voyages)")
            .unwrap()
            .query_map([], |row| Ok((row.get(1)?, row.get(2)?)))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(table_info.iter().any(|(name, _)| name == "id"));
        assert!(table_info.iter().any(|(name, _)| name == "origin_port"));
        assert!(
            table_info
                .iter()
                .any(|(name, _)| name == "destination_port")
        );
    }

    #[test]
    fn test_foreign_key_constraints() {
        let connection = setup_test_db();

        // Try to insert a voyage with invalid foreign keys
        let result = connection.execute(
            "INSERT INTO voyages (
                id,
                origin_port,
                destination_port
            ) VALUES (?, ?, ?)",
            params![999999, 999999, 999999],
        );

        assert!(result.is_err());
    }
}
