use std::fmt;

pub struct PgPlaceholdersSeq(pub usize);

impl fmt::Display for PgPlaceholdersSeq {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 { return Ok(()); }

        write!(f, "$1")?;
        for i in 2..=self.0 { write!(f, ",${}", i)? }

        Ok(())
    }

}
