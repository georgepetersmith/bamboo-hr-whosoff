use chrono::NaiveDate;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(time: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> {
    time.to_string().serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDate, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    Ok(NaiveDate::parse_from_str(&time, "%Y-%m-%d").map_err(D::Error::custom)?)
}
