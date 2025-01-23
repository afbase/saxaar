use crate::traits::FromRow;
use anyhow::Result;
use models::{place::Place, route_analysis::RouteAnalysis, temporal_pattern::TemporalPattern};
use rusqlite::{Connection, Result as SqlResult, Row, params};
use std::rc::Rc;

impl FromRow for TemporalPattern {
    fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(TemporalPattern {
            year: row.get("year")?,
            month: row.get("month")?,
            voyage_count: row.get("voyage_count")?,
            total_embarked: row.get("total_embarked")?,
            total_disembarked: row.get("total_disembarked")?,
        })
    }
}

/// Helper function to execute a query and collect results into a vector
///
/// # Arguments
/// * `database_connection` - Reference to the database connection
/// * `query` - SQL query string
/// * `params` - Query parameters
///
/// # Returns
/// * `Result<Vec<T>>` - Vector of results or an error
fn query_rows<T: FromRow>(
    database_connection: &Connection,
    query: &str,
    params: &[&dyn rusqlite::ToSql],
) -> Result<Vec<T>> {
    let mut statement = database_connection.prepare(query)?;
    let rows = statement.query_map(params, |row| T::from_row(row))?;
    Ok(rows.collect::<SqlResult<Vec<_>>>()?)
}

/// Analyze voyage routes between places
///
/// Provides detailed statistics about voyages between two places,
/// including total numbers, averages, and mortality rates.
///
/// # Arguments
/// * `database_connection` - Reference to the database connection
/// * `origin` - The origin place
/// * `destination` - The destination place
///
/// # Returns
/// * `Result<RouteAnalysis>` - Analysis results or an error
pub fn analyze_route(
    database_connection: &Rc<Connection>,
    origin: &Place,
    destination: &Place,
) -> Result<RouteAnalysis> {
    let sql_query = "
        WITH matching_voyages AS (
            SELECT 
                v.*,
                julianday(disembark_date) - julianday(embark_date) as journey_days
            FROM voyages v
            WHERE (
                v.origin_port = ?1 
                OR v.origin_region = ?1 
                OR v.origin_broad_region = ?1
            )
            AND (
                v.destination_port = ?2 
                OR v.destination_region = ?2 
                OR v.destination_broad_region = ?2
            )
            AND embark_date IS NOT NULL 
            AND disembark_date IS NOT NULL
        )
        SELECT 
            COUNT(*) as total_voyages,
            SUM(slaves_embarked) as total_embarked,
            SUM(slaves_disembarked) as total_disembarked,
            AVG(journey_days) as avg_journey_days,
            CASE 
                WHEN SUM(slaves_embarked) > 0 
                THEN 1.0 - (CAST(SUM(slaves_disembarked) AS FLOAT) / SUM(slaves_embarked))
                ELSE 0 
            END as mortality_rate
        FROM matching_voyages";

    let mut statement = database_connection.prepare(sql_query)?;
    let row = statement.query_row(params![origin.value, destination.value], |row| {
        Ok(RouteAnalysis {
            origin_place: origin.clone(),
            destination_place: destination.clone(),
            total_voyages: row.get(0)?,
            total_embarked: row.get(1)?,
            total_disembarked: row.get(2)?,
            average_journey_days: row.get(3)?,
            mortality_rate: row.get(4)?,
        })
    })?;

    Ok(row)
}

/// Analyze temporal patterns of voyages
///
/// Provides a breakdown of voyage patterns over time, with optional
/// monthly granularity.
///
/// # Arguments
/// * `database_connection` - Reference to the database connection
/// * `by_month` - Whether to break down by month
/// * `start_year` - Optional start year for the analysis
/// * `end_year` - Optional end year for the analysis
///
/// # Returns
/// * `Result<Vec<TemporalPattern>>` - Temporal analysis results or an error
pub fn analyze_temporal_patterns(
    database_connection: &Rc<Connection>,
    by_month: bool,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> Result<Vec<TemporalPattern>> {
    let date_format = if by_month {
        "strftime('%Y-%m', embark_date)"
    } else {
        "strftime('%Y', embark_date)"
    };

    let year_conditions = match (start_year, end_year) {
        (Some(start), Some(end)) => format!(
            "WHERE cast(strftime('%Y', embark_date) as integer) BETWEEN {} AND {}",
            start, end
        ),
        (Some(start), None) => format!(
            "WHERE cast(strftime('%Y', embark_date) as integer) >= {}",
            start
        ),
        (None, Some(end)) => format!(
            "WHERE cast(strftime('%Y', embark_date) as integer) <= {}",
            end
        ),
        (None, None) => String::new(),
    };

    let sql_query = format!(
        "SELECT
            cast(strftime('%Y', embark_date) as integer) as year,
            {month_select}
            COUNT(*) as voyage_count,
            SUM(slaves_embarked) as total_embarked,
            SUM(slaves_disembarked) as total_disembarked
        FROM voyages
        {year_conditions}
        GROUP BY {date_format}
        ORDER BY {date_format}",
        month_select = if by_month {
            "cast(strftime('%m', embark_date) as integer) as month,"
        } else {
            ""
        },
        year_conditions = year_conditions,
        date_format = date_format
    );

    query_rows(database_connection, &sql_query, &[])
}
