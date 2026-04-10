use darling::Error;
use quote::quote;
use syn::{Data, Fields};

use crate::table::field::generate_field_binding;

mod args;
mod field;

pub fn from_pdf_table(input: &syn::DeriveInput) -> Result<proc_macro::TokenStream, Error> {
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(Error::custom(
                    "FromPdfTable only works on structs with named fields",
                ));
            }
        },
        _ => {
            return Err(Error::custom("FromPdfTable can only be applied on structs"));
        }
    };

    let bindings = fields
        .iter()
        .map(generate_field_binding)
        .collect::<Vec<_>>();

    Ok(quote! {}.into())
}
