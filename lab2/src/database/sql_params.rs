use pg::types::ToSql;

#[derive(Debug)]
pub struct SqlParams<'a>(Vec<&'a dyn ToSql>);

impl<'a> SqlParams<'a> {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn push(&mut self, val: &'a dyn ToSql) -> usize {
        self.0.push(val);
        self.0.len()
    }

    pub fn extend(&mut self, vals: impl IntoIterator<Item = &'a dyn ToSql>) {
        self.0.extend(vals);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_slice(&self) -> &[&dyn ToSql] {
        self.0.as_slice()
    }
}
