use rusqlite::{Result as SqlResult, Row};
/// A trait for converting database rows into structured types
///
/// This trait provides a consistent interface for deserializing
/// database rows into our domain types. Implementations should
/// handle all necessary type conversions and validations.
pub trait FromRow: Sized {
    /// Converts a database row into the implementing type
    ///
    /// # Arguments
    /// * `row` - A reference to the database row
    ///
    /// # Returns
    /// * `SqlResult<Self>` - The converted type or an error
    fn from_row(row: &Row) -> SqlResult<Self>;
}
