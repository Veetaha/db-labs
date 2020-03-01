use pg::types::ToSql;
use soa_derive::{StructOfArray};

#[derive(StructOfArray)]
pub struct ColData {
    pub col: String,
    pub val: Box<dyn ToSql>
}

impl ColData {
    /// Create ColData that owns `val` and `col` by saving it on the heap
    pub fn with_boxed<TVal>(col: &str, val: TVal) -> Self
    where
        TVal: 'static + ToSql
    {
        ColData { col: col.to_owned(), val: Box::new(val) }
    }

    /// Push new ColData to `ColDataVec` with boxed col name and value if and only if
    /// `value.is_some()`
    pub fn try_push<T: 'static + ToSql>(col_upds: &mut ColDataVec, col: &str, val: Option<T>) {
        if let Some(val) = val {
            col_upds.push(ColData::with_boxed(col, val));
        }
    }
}
