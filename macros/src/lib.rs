use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod table;

#[proc_macro_derive(FromPdfTable, attributes(table))]
pub fn from_pdf_table(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match table::from_pdf_table(input) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}
