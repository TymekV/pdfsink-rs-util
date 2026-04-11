use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident};

use crate::util::extract_option_inner;

pub fn generate_field_binding(
    (index, field): (usize, &Field),
) -> Result<(Ident, TokenStream), darling::Error> {
    let field_ident = field
        .ident
        .as_ref()
        .ok_or_else(|| syn::Error::new_spanned(field.clone(), "expected a named field"))?;

    let field_name_str = field_ident.to_string();

    let ty = &field.ty;

    let binding = if let Some(inner_ty) = extract_option_inner(ty) {
        match quote!(#inner_ty).to_string().as_str() {
            "String" => quote! {
                let #field_ident: #ty = row
                    .get(#index)
                    .ok_or(::pdfsink_rs_util::FromTableError::ColumnNotFound(::pdfsink_rs_util::ColumnNotFound {
                        column: #field_name_str,
                    }))?
                    .clone();
            },
            "i32" | "i64" | "i128" | "u32" | "u64" | "u128" | "f32" | "f64" | "f128" => quote! {
                let #field_ident: #ty = row
                    .get(#index)
                    .ok_or(::pdfsink_rs_util::FromTableError::ColumnNotFound(::pdfsink_rs_util::ColumnNotFound {
                        column: #field_name_str,
                    }))?
                    .clone()
                    .map(|v| v.replace(',', ".").replace(' ', "").parse())
                    .transpose()?;
            },
            _ => {
                return Err(
                    darling::Error::custom(format!("unsupported type: {}", quote!(#ty)))
                        .with_span(ty),
                );
            }
        }
    } else {
        match quote!(#ty).to_string().as_str() {
            "String" => quote! {
                let #field_ident: #ty = row
                    .get(#index)
                    .ok_or(::pdfsink_rs_util::FromTableError::ColumnNotFound(::pdfsink_rs_util::ColumnNotFound {
                        column: #field_name_str,
                    }))?
                    .clone()
                    .ok_or(::pdfsink_rs_util::FromTableError::MissingValue(::pdfsink_rs_util::MissingValue {
                        column: #field_name_str,
                    }))?;
            },
            "i32" | "i64" | "i128" | "u32" | "u64" | "u128" | "f32" | "f64" | "f128" => quote! {
                let #field_ident: #ty = row
                    .get(#index)
                    .ok_or(::pdfsink_rs_util::FromTableError::ColumnNotFound(::pdfsink_rs_util::ColumnNotFound {
                        column: #field_name_str,
                    }))?
                    .clone()
                    .ok_or(::pdfsink_rs_util::FromTableError::MissingValue(::pdfsink_rs_util::MissingValue {
                        column: #field_name_str,
                    }))?
                    .replace(',', ".")
                    .replace(' ', "")
                    .parse()?;
            },
            _ => {
                return Err(
                    darling::Error::custom(format!("unsupported type: {}", quote!(#ty)))
                        .with_span(ty),
                );
            }
        }
    };

    Ok((field_ident.clone(), binding))
}
