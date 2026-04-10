use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn strict(bindings: &[(Ident, TokenStream)]) -> TokenStream {
    let idents = bindings.iter().map(|b| &b.0);
    let field_bindings = bindings.iter().map(|b| &b.1);

    quote! {
        let rows = table
            .iter()
            .map(|row| -> ::core::result::Result<Self, ::pdfsink_rs_util::table::FromTableError> {
                #(#field_bindings)*
                ::core::result::Result::Ok(Self { #(#idents),* })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }
}
