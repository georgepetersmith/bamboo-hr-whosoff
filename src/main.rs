use chrono::{NaiveDate, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::env;

mod date_serializer;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Calendar {
    #[serde(rename = "$value")]
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum ItemType {
    TimeOff,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    r#type: ItemType,
    request: Request,
    employee: Employee,
    #[serde(with = "date_serializer")]
    start: NaiveDate,
    #[serde(with = "date_serializer")]
    end: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Request {
    id: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Employee {
    id: u32,
    #[serde(rename = "$value")]
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let today = Utc::now().naive_utc().date();
    let whos_off_today = get_whos_off(today)?;
    println!("{:#?}", whos_off_today);

    Ok(())
}

fn get_whos_off(date: NaiveDate) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    // This can be a random string. The Bamboo HR API only uses the API key as the username.
    const PASSWORD: &str = "x";

    const API_KEY_ENV: &str = "BAMBOO_HR_API_KEY";
    let api_key = env::var(API_KEY_ENV)
        .expect("Could not find the value for the environment variable BAMBOO_HR_API_KEY");

    const DOMAIN_KEY: &str = "BAMBOO_HR_DOMAIN";
    let domain = env::var(DOMAIN_KEY)
        .expect("Could not find the value for the environment variable BAMBOO_HR_DOMAIN");

    let base_url = format!("https://api.bamboohr.com/api/gateway.php/{domain}/v1");
    let client = Client::new();

    let request = client
        .get(format!("{base_url}/time_off/whos_out"))
        .basic_auth(api_key, Some(PASSWORD));

    let response_body = request.send()?.text()?;
    let response: Calendar = from_str(response_body.as_str())?;

    let filtered_response = response
        .items
        .into_iter()
        .filter(|x| x.start <= date && date <= x.end)
        .collect::<Vec<Item>>();

    Ok(filtered_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let response_body = r#"
            <calendar>
             <item type="timeOff">
              <request id="10422"/>
              <employee id="13">George Smith</employee>
              <start>2023-08-29</start>
              <end>2023-09-25</end>
             </item>
             <item type="timeOff">
              <request id="10423"/>
              <employee id="14">Peter Smith</employee>
              <start>2023-09-12</start>
              <end>2023-09-14</end>
             </item>
            </calendar>"#;

        let expected = Calendar {
            items: Vec::from([
                Item {
                    r#type: ItemType::TimeOff,
                    request: Request { id: 10422 },
                    employee: Employee {
                        id: 13,
                        name: String::from("George Smith"),
                    },
                    start: NaiveDate::from_ymd_opt(2023, 8, 29).unwrap(),
                    end: NaiveDate::from_ymd_opt(2023, 9, 25).unwrap(),
                },
                Item {
                    r#type: ItemType::TimeOff,
                    request: Request { id: 10423 },
                    employee: Employee {
                        id: 14,
                        name: String::from("Peter Smith"),
                    },
                    start: NaiveDate::from_ymd_opt(2023, 9, 12).unwrap(),
                    end: NaiveDate::from_ymd_opt(2023, 9, 14).unwrap(),
                },
            ]),
        };

        let serialised_response: Calendar = from_str(response_body).unwrap();
        assert_eq!(serialised_response, expected);
    }
}
