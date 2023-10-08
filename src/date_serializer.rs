use chrono::NaiveDate;
use serde::{de::Error, Deserialize, Deserializer};

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&time, "%Y-%m-%d").map_err(D::Error::custom)
}
