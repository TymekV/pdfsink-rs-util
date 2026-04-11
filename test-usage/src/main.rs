use pdfsink_rs_util::{FromPdfTable, ValidateTable};

#[derive(FromPdfTable, ValidateTable, Debug)]
struct Test {
    #[column(name = "AAA", exact)]
    a: String,
    b: f64,
    c: Option<i32>,
}

fn main() {
    let table = vec![vec![Some("AAA".to_string()), Some("0,0".to_string()), None]];
    let a = Test::try_parse_table(&table).unwrap();

    dbg!(&a);
}
