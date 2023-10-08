use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::date_serializer;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Calendar {
    #[serde(rename = "$value")]
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    TimeOff,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub r#type: ItemType,
    pub request: Request,
    pub employee: Employee,
    #[serde(with = "date_serializer")]
    pub start: NaiveDate,
    #[serde(with = "date_serializer")]
    pub end: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Request {
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    pub id: u32,
    #[serde(rename = "$value")]
    pub name: String,
}
