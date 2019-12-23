pub trait GetTable {
    type Table;

    fn get_table(&self) -> Table;
}
