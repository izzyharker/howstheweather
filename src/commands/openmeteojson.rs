use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Cities {
    pub city: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Daily {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub precipitation_probability_max: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    pub temperature_2m: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub value: f64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Measurements<T> {
    pub data: Vec<T>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Date {
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub elevation: f64,
    pub current: Current,
    pub hourly: Hourly,
    pub daily: Daily,
}
