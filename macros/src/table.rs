use darling::Error;
use syn::{Data, Fields};

mod args;

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

    todo!()
}
