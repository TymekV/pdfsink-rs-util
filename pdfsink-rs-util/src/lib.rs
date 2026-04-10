pub mod macros;
pub mod table;

// #[derive(FromPdfTable)]
// struct Test {
//     a: String,
//     b: f64,
// }

// impl FromPdfTable for Test {
//     fn try_parse_strict(table: &table::Table) -> Result<Vec<Self>, table::FromTableError> {
//         let rows = table
//             .iter()
//             .map(|row| -> Result<Self, FromTableError> {
//                 let a: String = row
//                     .get(0)
//                     .ok_or(FromTableError::ColumnNotFound { column: "a" })?
//                     .clone()
//                     .ok_or(FromTableError::MissingValue { column: "a" })?;

//                 Ok(Self { a, b: 0.0 })
//             })
//             .collect::<Result<Vec<_>, _>>()?;

//         Ok(rows)
//     }

//     fn try_parse(table: &table::Table) -> Vec<Self> {
//         todo!()
//     }
// }
