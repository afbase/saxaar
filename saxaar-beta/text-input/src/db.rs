use csv::ReaderBuilder;
use rusqlite::{Connection, Result, params};
use strsim::{damerau_levenshtein, sorensen_dice};

const PORTS: &str = include_str!("../../fixtures/geography.csv");

#[derive(Clone, PartialEq)]
pub struct Port {
    pub broad_region: String,
    pub specific_region: String,
    pub name: String,
    pub value: i32,
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;

    // Create ports table with new structure
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            broad_region TEXT NOT NULL,
            specific_region TEXT NOT NULL,
            name TEXT NOT NULL,
            value INTEGER NOT NULL
        )",
        [],
    )?;

    // Parse CSV and insert ports
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(PORTS.as_bytes());

    for result in rdr.records() {
        if let Ok(record) = result {
            // Skip if we don't have all required fields
            if record.len() < 6 {
                continue;
            }

            let value: i32 = record[0].parse().unwrap_or(0);
            let broad_region = &record[1];
            let specific_region = &record[3];
            let name = &record[5];

            conn.execute(
                "INSERT INTO ports (broad_region, specific_region, name, value) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![broad_region, specific_region, name, value],
            )?;
        }
    }

    Ok(conn)
}

pub fn search_ports(conn: &Connection, search_term: &str) -> Result<Vec<Port>> {
    let mut stmt = conn.prepare("SELECT broad_region, specific_region, name, value FROM ports")?;

    let port_iter = stmt.query_map([], |row| {
        Ok(Port {
            broad_region: row.get(0)?,
            specific_region: row.get(1)?,
            name: row.get(2)?,
            value: row.get(3)?,
        })
    })?;

    let mut port_matches = Vec::new();
    let search_term = search_term.to_lowercase();

    for port in port_iter {
        let port = port?;
        let port_name_lower = port.name.to_lowercase();

        let distance = damerau_levenshtein(&search_term, &port_name_lower);

        port_matches.push((distance, port));
    }

    // Sort by distance
    port_matches.sort_by_key(|(distance, _)| *distance);

    // Take top 4 matches
    let ports = port_matches
        .into_iter()
        .take(4)
        .map(|(_, port)| port)
        .collect();

    Ok(ports)
}

pub fn search_ports_with_region(conn: &Connection, search_term: &str) -> Result<Vec<Port>> {
    let mut stmt = conn.prepare("SELECT broad_region, specific_region, name, value FROM ports")?;

    let port_iter = stmt.query_map([], |row| {
        Ok(Port {
            broad_region: row.get(0)?,
            specific_region: row.get(1)?,
            name: row.get(2)?,
            value: row.get(3)?,
        })
    })?;

    let search_term = search_term.to_lowercase();
    let mut port_matches = Vec::new();

    for port in port_iter {
        let port = port?;
        let port_name_lower = port.name.to_lowercase();
        let port_specific_region_lower = port.specific_region.to_lowercase();
        let port_broad_region_lower = port.broad_region.to_lowercase();

        // Calculate distances for name and regions
        let name_distance = damerau_levenshtein(&search_term, &port_name_lower);
        let specific_region_distance =
            damerau_levenshtein(&search_term, &port_specific_region_lower);
        let broad_region_distance = damerau_levenshtein(&search_term, &port_broad_region_lower);

        let name_similarity = sorensen_dice(&search_term, &port_name_lower);
        let specific_region_similarity = sorensen_dice(&search_term, &port_specific_region_lower);
        let broad_region_similarity = sorensen_dice(&search_term, &port_broad_region_lower);

        // Combined score with weights
        let name_weight = 0.7;
        let specific_region_weight = 0.2;
        let broad_region_weight = 0.1;

        let combined_distance = name_distance as f64 * name_weight
            + specific_region_distance as f64 * specific_region_weight
            + broad_region_distance as f64 * broad_region_weight;

        let combined_similarity = 1.0
            - (name_similarity * name_weight
                + specific_region_similarity * specific_region_weight
                + broad_region_similarity * broad_region_weight);

        let mut combined_score = (combined_distance.powi(2) + combined_similarity.powi(2)).sqrt();

        // Apply bonuses for different types of matches
        if port_name_lower.starts_with(&search_term) {
            combined_score -= 1000.0;
        }
        if port_name_lower.contains(&search_term) {
            combined_score -= 500.0;
        }
        if port_specific_region_lower.starts_with(&search_term) {
            combined_score -= 100.0;
        }
        if port_broad_region_lower.starts_with(&search_term) {
            combined_score -= 50.0;
        }

        // Include results if they're reasonably close
        if name_distance <= 5 || specific_region_distance <= 3 || broad_region_distance <= 3 {
            port_matches.push((combined_score, port));
        }
    }

    // Sort by combined score
    port_matches.sort_by(|(score1, _), (score2, _)| {
        score1
            .partial_cmp(score2)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Take top 5 matches
    let ports = port_matches
        .into_iter()
        .take(5)
        .map(|(_, port)| port)
        .collect();

    Ok(ports)
}
