use thiserror::Error;

pub type Table = Vec<Vec<Option<String>>>;

#[derive(Debug, Error)]
pub enum FromTableError {
    #[error("A column could not be found: expected column `{column}`")]
    ColumnNotFound { column: &'static str },
}

/// This trait allows for conversions from a table detected by `pdfsink-rs` to the target struct.
pub trait FromPdfTable: Sized {
    /// Tries to parse a table into the target struct.
    ///
    /// "Strict" means that if a row is malformed, the function will error out.
    fn try_parse_strict(table: &Table) -> Result<Self, FromTableError>;

    /// Tries to parse a table into the target struct.
    ///
    /// Ignores rows that do not match.
    fn try_parse(table: &Table) -> Result<Self, FromTableError>;
}
