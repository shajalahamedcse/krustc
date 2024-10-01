pub trait DbInsertable {
    fn table_name() -> &'static str;
    fn to_insert_query(&self) -> (String, Vec<String>);
}
