use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

pub mod validator;

pub type Table = Vec<Vec<Option<String>>>;

#[derive(Debug, Error)]
pub enum MergeError {
    #[error("Previous row is missing")]
    PreviousRowMissing,
}

/// Merges malformed continuation rows into the previous row.
///
/// A row is treated as continuation if `required_column_index` is missing.
/// Non-empty continuation values are merged into the previous row, joining with `\n`
/// when both values are present.
pub fn merge_continuation_rows(
    table: &Table,
    required_column_index: usize,
) -> Result<Table, MergeError> {
    let mut merged: Table = Vec::with_capacity(table.len());

    for row in table {
        let has_required_value = row
            .get(required_column_index)
            .is_some_and(|value| value.is_some());

        if has_required_value || merged.is_empty() {
            merged.push(row.clone());
            continue;
        }

        let previous = merged.last_mut().ok_or(MergeError::PreviousRowMissing)?;

        if previous.len() < row.len() {
            previous.resize(row.len(), None);
        }

        for (index, cell) in row.iter().enumerate() {
            let Some(cell_value) = cell else {
                continue;
            };

            match previous.get_mut(index) {
                Some(Some(previous_value)) if !previous_value.is_empty() => {
                    previous_value.push('\n');
                    previous_value.push_str(cell_value);
                }
                Some(slot) => {
                    *slot = Some(cell_value.clone());
                }
                None => {
                    previous.push(Some(cell_value.clone()));
                }
            }
        }
    }

    Ok(merged)
}

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

    #[error(transparent)]
    MergeError(#[from] MergeError),
}

/// This trait allows for conversions from a table detected by `pdfsink-rs` to the target struct.
pub trait FromPdfTable: Sized {
    /// Tries to parse a table into the target struct.
    ///
    /// If a row is malformed, the function will error out.
    fn try_parse_table(table: &Table) -> Result<Vec<Self>, FromTableError>;

    /// Tries to parse a table after merging continuation rows into the previous row.
    ///
    /// A row is treated as continuation if `required_column_index` is missing.
    fn try_parse_table_with_merged_continuations(
        table: &Table,
        required_column_index: usize,
    ) -> Result<Vec<Self>, FromTableError> {
        let table = merge_continuation_rows(table, required_column_index)?;
        Self::try_parse_table(&table)
    }

    /// Tries to parse a table into the target struct.
    ///
    /// Ignores rows that do not match.
    fn parse_table(table: &Table) -> Vec<Self>;

    /// Tries to parse a table after merging continuation rows into the previous row.
    ///
    /// A row is treated as continuation if `required_column_index` is missing.
    fn parse_table_with_merged_continuations(
        table: &Table,
        required_column_index: usize,
    ) -> Result<Vec<Self>, MergeError> {
        let table = merge_continuation_rows(table, required_column_index)?;
        Ok(Self::parse_table(&table))
    }
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

#[cfg(test)]
mod tests {
    use super::merge_continuation_rows;

    #[test]
    fn merges_rows_without_required_column_into_previous_row() {
        let table = vec![
            vec![
                Some("1".to_string()),
                Some("Foo".to_string()),
                Some("First line".to_string()),
            ],
            vec![None, None, Some("Second line".to_string())],
            vec![
                Some("2".to_string()),
                Some("Bar".to_string()),
                Some("Another".to_string()),
            ],
        ];

        let merged = merge_continuation_rows(&table, 0).expect("merge failed");

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0][0].as_deref(), Some("1"));
        assert_eq!(merged[0][2].as_deref(), Some("First line\nSecond line"));
        assert_eq!(merged[1][0].as_deref(), Some("2"));
    }

    #[test]
    fn keeps_first_row_when_it_is_missing_required_column() {
        let table = vec![
            vec![None, Some("orphan".to_string())],
            vec![Some("1".to_string()), Some("real".to_string())],
        ];

        let merged = merge_continuation_rows(&table, 0).expect("merge failed");

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0][1].as_deref(), Some("orphan"));
        assert_eq!(merged[1][0].as_deref(), Some("1"));
    }
}
