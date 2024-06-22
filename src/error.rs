use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::path::PathBuf;

use crate::parser::Rule;

pub type Result<'a, T> = std::result::Result<T, QuexError<'a>>;

#[derive(thiserror::Error)]
pub enum QuexError<'a> {
    #[error("Invalid Ethiopian date: \"{}\"", .1)]
    EthiopianDate(#[source] zemen::error::Error, &'a str),

    #[error("Invalid Gregorian date: \"{1}\"")]
    GregorianDate(#[source] time::error::ComponentRange, &'a str),

    #[error("Recurring monthly out of range: \"{1}\"")]
    RecurringMonthly(#[source] time::error::ComponentRange, &'a str),

    #[error("Parse error")]
    Parse(#[source] Box<pest::error::Error<Rule>>),
}

impl<'a> Debug for QuexError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self)?;

        if let Some(source) = self.source() {
            write!(f, "Caused by:\n\t{}", source)?;
        }

        Ok(())
    }
}

pub fn ethiopian_date<'a>(r: &'a str) -> impl FnOnce(zemen::error::Error) -> QuexError<'a> {
    return |e: zemen::error::Error| QuexError::EthiopianDate(e, r);
}

pub fn gregorian_date<'a>(r: &'a str) -> impl FnOnce(time::error::ComponentRange) -> QuexError<'a> {
    return |e: time::error::ComponentRange| QuexError::GregorianDate(e, r);
}

pub fn recurring_monthly<'a>(
    r: &'a str,
) -> impl FnOnce(time::error::ComponentRange) -> QuexError<'a> {
    return |e: time::error::ComponentRange| QuexError::RecurringMonthly(e, r);
}

#[derive(thiserror::Error)]
#[error("I/O error while processing file: {file}")]
pub struct QuexIoError {
    #[source]
    source: std::io::Error,
    file: PathBuf,
}

impl QuexIoError {
    pub fn new(file: PathBuf) -> impl FnOnce(std::io::Error) -> Self {
        return |source| QuexIoError { source, file };
    }
}

impl Debug for QuexIoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self)?;

        if let Some(source) = self.source() {
            write!(f, "Caused by:\n\t{}", source)?;
        }

        Ok(())
    }
}
