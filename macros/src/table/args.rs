use darling::FromField;

#[derive(FromField)]
#[darling(attributes(table))]
#[allow(dead_code)]
pub struct TableColumnArgs {
    pub column_name: String,

    #[darling(default)]
    pub exact: bool,
}
