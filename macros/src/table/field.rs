use darling::FromField;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident};

pub fn generate_field_binding(field: &Field) -> Result<(Ident, TokenStream), darling::Error> {
    let field_ident = field
        .ident
        .as_ref()
        .ok_or_else(|| syn::Error::new_spanned(field.clone(), "expected a named field"))?;

    let ty = &field.ty;
    let binding = match quote!(#ty).to_string().as_str() {
        "String" => quote! {
            let #field_ident: String = row.get(stringify!(#field_ident));
        },
        "i32" => quote! {
            let #field_ident: i32 = row.get(stringify!(#field_ident));
        },
        _ => {
            return Err(
                darling::Error::custom(format!("unsupported type: {}", quote!(#ty))).with_span(ty),
            );
        }
    };

    Ok((field_ident.clone(), binding))
}
