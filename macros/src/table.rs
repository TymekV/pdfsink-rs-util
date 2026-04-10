use darling::Error;
use quote::quote;
use syn::{Data, Fields};

use crate::table::{field::generate_field_binding, strict::strict};

mod args;
mod field;
mod strict;

pub fn from_pdf_table(input: &syn::DeriveInput) -> Result<proc_macro::TokenStream, Error> {
    let target = &input.ident;
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
        .enumerate()
        .map(generate_field_binding)
        .collect::<Result<Vec<_>, _>>()?;

    let strict = strict(&bindings);

    let impl_block = quote! {
        impl ::pdfsink_rs_util::FromPdfTable for #target {
            fn try_parse_table(table: &::pdfsink_rs_util::Table) -> ::core::result::Result<::std::vec::Vec<Self>, ::pdfsink_rs_util::FromTableError> {
                #strict
            }
        }
    };

    Ok(impl_block.into())
}
