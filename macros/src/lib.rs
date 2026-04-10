use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod from_table;
mod util;
mod validate_table;

#[proc_macro_derive(FromPdfTable)]
pub fn from_pdf_table(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match from_table::from_pdf_table(&input) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

#[proc_macro_derive(ValidateTable, attributes(column))]
pub fn validate_table(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match validate_table::validate_table(&input) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}
