use chrono::{NaiveDate, ParseError};
use serde::{de::Error, Deserialize, Deserializer};

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    parse_from_str(&time).map_err(D::Error::custom)
}

pub fn parse_from_str(arg: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(arg, "%Y-%m-%d")
}