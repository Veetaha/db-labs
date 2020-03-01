use std::fmt;
use std::ops::RangeInclusive;

pub struct PgPlaceholdersSeq(RangeInclusive<usize>);

impl PgPlaceholdersSeq {
    pub fn new(range: RangeInclusive<usize>) -> Self { Self(range) }
}

impl fmt::Display for PgPlaceholdersSeq {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start = *self.0.start();
        let end   = *self.0.end();

        if start > end { return Ok(()); }

        write!(f, "${}", start)?;
        for i in (start + 1)..=end {
            write!(f, ",${}", i)?
        }

        Ok(())
    }

}
