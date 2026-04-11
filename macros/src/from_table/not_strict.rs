use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn not_strict(bindings: &[(Ident, TokenStream)]) -> TokenStream {
    let idents = bindings.iter().map(|b| &b.0);
    let field_bindings = bindings.iter().map(|b| &b.1);

    quote! {
        table
            .iter()
            .map(|row| -> ::core::result::Result<Self, ::pdfsink_rs_util::FromTableError> {
                #(#field_bindings)*
                ::core::result::Result::Ok(Self { #(#idents),* })
            })
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>()
    }
}
