use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct TimeWrapper(time::Time);

impl TimeWrapper {
    pub fn from_hms(h: u8, m: u8, s: u8) -> Result<Self, time::error::ComponentRange> {
        Ok(TimeWrapper(time::Time::from_hms(h, m, s)?))
    }
}

impl serde::Serialize for TimeWrapper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl Deref for TimeWrapper {
    type Target = time::Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TimeWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = self.0;
        write!(f, "{:02}:{:02}:{:02} ", t.hour(), t.minute(), t.second())
    }
}
