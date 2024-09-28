#[cfg(feature = "eth")]
pub mod eth;

pub mod gre;

use pest::error::InputLocation;
use serde_derive::Serialize;

use std::{
    fmt::{self},
    ops::Deref,
};

pub type DateResult<D> = Result<Option<Event<D>>, LineError>;

// #[serde(bound = "T: MyTrait")] : helpful if we want to store the actual error in ParingError { error }
#[derive(Debug, Serialize)]
pub enum LineError {
    CantParseInput,
    InvalidValue(String),
    ParsingError {
        #[serde(skip_serializing)]
        error: String,
        message: String,
        column: ColumnLocation,
    },
}

#[derive(Debug)]
pub struct ColumnLocation(pub InputLocation);

impl ColumnLocation {
    pub fn new(loc: InputLocation) -> Self {
        Self(loc)
    }
}

impl Deref for ColumnLocation {
    type Target = InputLocation;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::Serialize for ColumnLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{:?}", self.0).serialize(serializer)
    }
}

pub trait DateInfo: fmt::Display + fmt::Debug {
    fn julian_day(&self) -> i32;
    fn pretty_print(&self) -> String;
}

#[derive(Debug)]
pub struct Event<C: DateInfo> {
    pub date: C,
    pub message: String,
}

impl<C: DateInfo> Event<C> {
    pub fn new(date: C, message: String) -> Self {
        Self { date, message }
    }
}
