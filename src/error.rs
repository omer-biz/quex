use std::error::Error;
use std::fmt;
use std::fmt::Debug;

use crate::parser::Rule;

#[derive(thiserror::Error)]
pub enum QuexError<'a> {
    #[error("Invalid Ethiopian date: \"{}\"", .1)]
    EthiopianDate(#[source] zemen::error::Error, &'a str),

    #[error("Invalid Gregorian date: \"{1}\"")]
    GregorianDate(#[source] time::error::ComponentRange, &'a str),

    #[error("Recurring monthly out of range: \"{1}\"")]
    RecurringMonthly(#[source] time::error::ComponentRange, &'a str),

    #[error("Parse error")]
    Parse(#[from] pest::error::Error<Rule>),
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
