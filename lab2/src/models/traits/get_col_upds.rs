use pg::types::ToSql;

pub struct ColUpd<'a> {
    pub col: &'a str,
    pub val: &'a dyn ToSql
}

pub trait GetColUpds {
    fn get_col_upds<'a>(&'a self) -> Vec<ColUpd<'a>>;
}
