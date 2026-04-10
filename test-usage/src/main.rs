use pdfsink_rs_util::macros::FromPdfTable;
use pdfsink_rs_util::table::FromPdfTable;

#[derive(FromPdfTable, Debug)]
struct Test {
    a: String,
    b: f64,
}

fn main() {
    let table = vec![vec![Some("AAA".to_string()), Some("0.0".to_string())]];
    let a = Test::try_parse_strict(&table).unwrap();
    dbg!(&a);
}
