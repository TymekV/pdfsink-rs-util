use darling::FromField;

#[derive(FromField)]
#[darling(attributes(column))]
#[allow(dead_code)]
pub struct TableColumnArgs {
    pub name: Option<String>,

    #[darling(default)]
    pub exact: bool,
}
