use chrono::{NaiveDate, Utc};
use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

mod date_serializer;

#[derive(Parser, Debug)]
struct Args {
    /// The date on which to check who is off.
    #[arg(short, long)]
    #[arg(value_parser = date_serializer::parse_from_str)]
    #[arg(default_value_t = Utc::now().naive_utc().date())]
    date: NaiveDate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WhosOff {
    name: String,
    #[serde(with = "date_serializer")]
    start: NaiveDate,
    #[serde(with = "date_serializer")]
    end: NaiveDate,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Who is off on {}:\r\n", &args.date);

    let mut whos_off = get_whos_off(args.date)?;
    whos_off.sort_by(|a, b| a.end.cmp(&b.end));
    whos_off
        .iter()
        .for_each(|x| println!("{} -> {} {}", &x.start, &x.end, &x.name));

    Ok(())
}

fn get_whos_off(date: NaiveDate) -> Result<Vec<WhosOff>, Box<dyn std::error::Error>> {
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
        .header("accept", "application/json")
        .basic_auth(api_key, Some(PASSWORD));

    let response: Vec<WhosOff> = request.send()?.json()?;

    let filtered_response = response
        .into_iter()
        .filter(|x| x.start <= date && date <= x.end)
        .collect::<Vec<WhosOff>>();

    Ok(filtered_response)
}