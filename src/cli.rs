use colored::*;
use core::result::Result;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::io;

// struct to deserialise JSON response from openweather API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    paramters: Parameters,
    wind: Wind,
    name: &'static str,
}

// struct to represent weather desc
#[derive(Debug, Deserialize)]
struct Weather {
    description: &'static str,
}

// struct to represent main weather parameters
#[derive(Debug, Deserialize)]
struct Parameters {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// struct to represent wind information
#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}

// function to get weather information from API
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = ""
}
