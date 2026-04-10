use crate::{Column, ColumnNotFound, Table, ValidateTableError, util::normalize};

pub struct TableValidator {
    target_columns: &'static [Column],
}

impl TableValidator {
    pub fn new(target_columns: &'static [Column]) -> Self {
        Self { target_columns }
    }

    pub fn table_matches_signature(&self, table: &Table) -> Result<(), ValidateTableError> {
        let header = table.first().ok_or(ValidateTableError::HeaderNotFound)?;

        header.iter().enumerate().try_for_each(
            |(i, column)| -> Result<(), ValidateTableError> {
                let expected_column = &self
                    .target_columns
                    .get(i)
                    .ok_or(ValidateTableError::TooManyColumns)?;

                let column =
                    column
                        .as_ref()
                        .ok_or(ValidateTableError::ColumnNotFound(ColumnNotFound {
                            column: expected_column.name,
                        }))?;

                let matches = if expected_column.exact {
                    column == expected_column.name
                } else {
                    normalize(column).contains(&normalize(expected_column.name))
                };

                match matches {
                    true => Ok(()),
                    false => Err(ValidateTableError::ColumnNotFound(ColumnNotFound {
                        column: expected_column.name,
                    })),
                }
            },
        )?;

        Ok(())
    }
}
