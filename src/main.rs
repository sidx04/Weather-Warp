use colored::*;
use configure::configure;
use dotenv::dotenv;
use std::env;

mod cli;
use crate::cli::{display, get_weather_info};

mod args;

mod configure;
fn main() {
    dotenv().ok();

    let args = args::Args::arg_parse();

    // If no command is provided, execute the `Weather` command by default
    let command = match args.command {
        Some(c) => c,
        None => {
            let default_city = Some(String::from("Kolkata"));
            let default_country = Some(String::from("IND"));
            args::Command::Weather {
                city: default_city,
                country: default_country,
            }
        }
    };

    match command {
        args::Command::Weather { city, country } => {
            let api_key = env::var("OPENWEATHER_API_KEY").unwrap();
            let api_url = env::var("OPENWEATHER_API_URL").unwrap();

            println!("{}", "Welcome to Weather Warp.".bright_cyan());

            // Extract the city and country from the Option
            let city = city.unwrap_or_else(|| String::from("Kolkata"));
            let country = country.unwrap_or_else(|| String::from("IND"));

            match get_weather_info(&city, &country, &api_url, &api_key) {
                Ok(response) => {
                    display(&response);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        }
        args::Command::Config { city, country } => {
            configure(city.unwrap(), country.unwrap());
        }
    }
}
