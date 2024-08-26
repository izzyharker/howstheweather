use csv;
use geolocation::{find, Locator};
use my_internet_ip;
use reqwest;
use serde::{Deserialize, Serialize};
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct Hourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Daily {
    time: Vec<String>,
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>,
    precipitation_probability_max: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Current {
    temperature_2m: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    value: f64,
}

#[derive(Serialize, Debug, Deserialize)]
struct Measurements<T> {
    data: Vec<T>,
}

#[derive(Serialize, Debug, Deserialize)]
struct Date {
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    timezone: String,
    elevation: f64,
    current: Current,
    hourly: Hourly,
    daily: Daily,
}

// Allows use of .await for async requests
#[tokio::main]
pub async fn weather(options: &[ResolvedOption]) -> String {
    let req_city = if let Some(ResolvedOption {
        value: ResolvedValue::String(city),
        ..
    }) = options.first()
    {
        format!("{}", city)
    } else {
        "Please provide a valid attachment".to_string()
    };

    let mut cities = csv::Reader::from_reader(File::open("./src/data/cities.csv").unwrap());

    println!("{:?}", cities.headers());

    let ip_address = match my_internet_ip::get() {
        Ok(ip) => ip,
        Err(e) => panic!("Could not get IP: {:?}", e),
    };

    let location: Locator = match find(&format!("{:?}", ip_address)) {
        Ok(loc) => loc,
        Err(e) => panic!("Issue with IP address: {:?}", e),
    };

    // TODO: find the city from dataset

    // Maybe switch to national weather service api at some point?
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m&hourly=temperature_2m&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&temperature_unit=fahrenheit&timezone=America%2FLos_Angeles&forecast_days=3", location.latitude, location.longitude);

    let response = reqwest::get(url).await.unwrap();

    let parsed = match response.status().is_success() {
        true => {
            let text = response.text().await.unwrap();
            // println!("Success! \n{:?}", text);

            let data: Weather = serde_json::from_str(&text).unwrap();
            data
        }
        _ => {
            panic!("Oh no! A wild error appears: {:?}", response.status());
        }
    };

    let retstr: String = format!(
        "Approximate Location (lat, long): {}, {}\nCurrent temperature: {}\nRequested city: {}",
        location.latitude, location.longitude, parsed.current.temperature_2m, req_city
    );

    retstr
}

pub fn register() -> CreateCommand {
    CreateCommand::new("weather")
        .description("Gets the weather for your city.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "city",
                "The city to get the weather of",
            )
            .required(true),
        )
}
