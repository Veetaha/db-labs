use pg::types::ToSql;
use soa_derive::{StructOfArray};

#[derive(StructOfArray)]
pub struct ColData {
    pub col: String,
    pub val: Box<dyn ToSql>
}

impl ColData {
    pub fn with_boxed<TVal>(col: &str, val: TVal) -> Self
    where
        TVal: 'static + ToSql
    {
        ColData { col: col.to_owned(), val: Box::new(val) }
    }
}
