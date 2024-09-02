use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TimeSpan {
    Unit(time::Time),
    Range(time::Time, time::Time),
}

impl TimeSpan {
    pub fn new_unit(t: time::Time) -> Self {
        TimeSpan::Unit(t)
    }

    pub fn new_range(t1: time::Time, t2: time::Time) -> Self {
        TimeSpan::Range(t1, t2)
    }
}

impl serde::Serialize for TimeSpan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TimeSpan::Unit(t) => t.to_string().serialize(serializer),
            TimeSpan::Range(t1, t2) => format!("{}-{}", t1, t2).serialize(serializer),
        }
    }
}

impl fmt::Display for TimeSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeSpan::Unit(t) => t.to_string().fmt(f),
            TimeSpan::Range(t1, t2) => format!("{}-{}", t1, t2).fmt(f),
        }
    }
}
