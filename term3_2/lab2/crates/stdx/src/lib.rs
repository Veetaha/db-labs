
use std::{cell::Cell, fmt};

pub fn display_with<F>(f: F) -> DisplayWith<F>
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result
{
    DisplayWith(f)
}

pub struct DisplayWith<F>(F);

impl<F> fmt::Display for DisplayWith<F>
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

// Copied from https://github.com/rust-analyzer/rust-analyzer/blob/master/crates/stdx/src/lib.rs
pub trait SepBy: Sized {
    /// Returns an `impl fmt::Display`, which joins elements via a separator.
    fn sep_by<'a>(self, sep: &'a str) -> SepByBuilder<'a, Self>;
}

impl<I> SepBy for I
where
    I: Iterator,
    I::Item: fmt::Display,
{
    fn sep_by<'a>(self, sep: &'a str) -> SepByBuilder<'a, Self> {
        SepByBuilder::new(sep, self)
    }
}

pub struct SepByBuilder<'a, I> {
    sep: &'a str,
    prefix: &'a str,
    suffix: &'a str,
    iter: Cell<Option<I>>,
}

impl<'a, I> SepByBuilder<'a, I> {
    fn new(sep: &'a str, iter: I) -> SepByBuilder<'a, I> {
        SepByBuilder { sep, prefix: "", suffix: "", iter: Cell::new(Some(iter)) }
    }

    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn suffix(mut self, suffix: &'a str) -> Self {
        self.suffix = suffix;
        self
    }

    /// Set both suffix and prefix.
    pub fn surround_with(self, prefix: &'a str, suffix: &'a str) -> Self {
        self.prefix(prefix).suffix(suffix)
    }
}


impl<I> fmt::Display for SepByBuilder<'_, I>
where
    I: Iterator,
    I::Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.prefix)?;
        let mut first = true;
        for item in self.iter.take().unwrap() {
            if !first {
                f.write_str(self.sep)?;
            }
            first = false;
            fmt::Display::fmt(&item, f)?;
        }
        f.write_str(self.suffix)?;
        Ok(())
    }
}
