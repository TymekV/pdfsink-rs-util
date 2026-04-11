use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

pub mod validator;

pub type Table = Vec<Vec<Option<String>>>;

#[derive(Error, Debug)]
#[error("A column could not be found: expected column `{column}`")]
pub struct ColumnNotFound {
    pub column: &'static str,
}

#[derive(Error, Debug)]
#[error("A value is missing in column `{column}`")]
pub struct MissingValue {
    pub column: &'static str,
}

#[derive(Debug, Error)]
pub enum FromTableError {
    #[error(transparent)]
    ColumnNotFound(#[from] ColumnNotFound),

    #[error(transparent)]
    MissingValue(#[from] MissingValue),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),
}

/// This trait allows for conversions from a table detected by `pdfsink-rs` to the target struct.
pub trait FromPdfTable: Sized {
    /// Tries to parse a table into the target struct.
    ///
    /// If a row is malformed, the function will error out.
    fn try_parse_table(table: &Table) -> Result<Vec<Self>, FromTableError>;

    /// Tries to parse a table into the target struct.
    ///
    /// Ignores rows that do not match.
    fn parse_table(table: &Table) -> Vec<Self>;
}

#[derive(Debug, Error)]
pub enum ValidateTableError {
    #[error(transparent)]
    ColumnNotFound(#[from] ColumnNotFound),

    #[error("Too many columns")]
    TooManyColumns,

    #[error("Header not found")]
    HeaderNotFound,
}

/// This trait allows to validate that a table detected by `pdfsink-rs` has the correct structure.
pub trait ValidateTable {
    /// Validates that a table has the correct structure.
    fn validate_table(table: &Table) -> Result<(), ValidateTableError>;
}
