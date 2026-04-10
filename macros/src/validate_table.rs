use darling::{Error, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, Fields};

use crate::validate_table::args::TableColumnArgs;

mod args;
mod column;

pub fn validate_table(input: &syn::DeriveInput) -> Result<proc_macro::TokenStream, Error> {
    let target = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(Error::custom(
                    "ValidateTable only works on structs with named fields",
                ));
            }
        },
        _ => {
            return Err(Error::custom(
                "ValidateTable can only be applied on structs",
            ));
        }
    };

    let target_columns = fields
        .iter()
        .map(|field| -> Result<TokenStream, Error> {
            let args = TableColumnArgs::from_field(field)?;
            let exact = args.exact;

            let column_name = args.name.unwrap_or(
                field
                    .ident
                    .as_ref()
                    .ok_or_else(|| {
                        syn::Error::new_spanned(field.clone(), "expected a named field")
                    })?
                    .to_string(),
            );

            let column = quote! {
                ::pdfsink_rs_util::Column {
                    name: #column_name,
                    exact: #exact,
                }
            };

            Ok(column)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let check = quote! {
        const COLUMNS: &'static [::pdfsink_rs_util::Column] = &[#(#target_columns),*];
        let validator = ::pdfsink_rs_util::validator::TableValidator::new(COLUMNS);
        validator.table_matches_signature(table)?;
        Ok(())
    };

    let impl_block = quote! {
        impl ::pdfsink_rs_util::ValidateTable for #target {
            fn validate_table(table: &::pdfsink_rs_util::Table) -> ::core::result::Result<(), ::pdfsink_rs_util::ValidateTableError> {
                #check
            }
        }
    };

    Ok(impl_block.into())
}
