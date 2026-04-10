use pdfsink_rs_util_macros::FromPdfTable;

pub mod table;

#[derive(FromPdfTable)]
struct Test {
    a: String,
}
