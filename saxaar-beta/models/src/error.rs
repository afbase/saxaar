use crate::{place::Place, port::Port, voyage::Voyage};
use csv::Error as CsvError;
use rusqlite::Error as SqliteError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbInitError {
    #[error("Failed to insert Port {0:?} into sqlite table")]
    InsertPortRow(Port),
    #[error("Failed to insert Voyage {0:?} into sqlite table")]
    InsertVoyageRow(Voyage),
    #[error("Failed to deserialize the csv {0:?}")]
    CsvDeserialize(CsvError),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to insert Place {0:?} into database")]
    InsertPlaceRow(Place),

    #[error("Failed to insert Voyage {0:?} into database")]
    InsertVoyageRow(Voyage),

    #[error("Failed to deserialize CSV data: {0}")]
    CsvDeserialize(#[from] CsvError),

    #[error("Database error: {0}")]
    SqliteError(#[from] SqliteError),

    #[error("Failed to initialize database: {0}")]
    InitializationError(String),

    #[error("Invalid place type: {0}")]
    InvalidPlaceType(String),

    #[error("Failed to deserialize database row: {0}")]
    RowDeserializationError(String),

    #[error("Search error: {0}")]
    SearchError(String),

    #[error("Query error: {0}")]
    QueryError(String),
}

// Result type alias for our database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

// Helper functions for error conversion
impl DatabaseError {
    pub fn from_string<T: ToString>(error: T) -> Self {
        DatabaseError::InitializationError(error.to_string())
    }

    pub fn search_error<T: ToString>(error: T) -> Self {
        DatabaseError::SearchError(error.to_string())
    }

    pub fn query_error<T: ToString>(error: T) -> Self {
        DatabaseError::QueryError(error.to_string())
    }
}

// Implement conversion from standard Error types
impl From<std::io::Error> for DatabaseError {
    fn from(error: std::io::Error) -> Self {
        DatabaseError::InitializationError(error.to_string())
    }
}

impl From<SerdeJsonError> for DatabaseError {
    fn from(error: SerdeJsonError) -> Self {
        DatabaseError::RowDeserializationError(error.to_string())
    }
}
