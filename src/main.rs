use crate::cli::display;
use crate::cli::get_weather_info;
use colored::*;
use dotenv::dotenv;
use std::env;
use std::io;

mod cli;
fn main() {
    dotenv().ok();

    let api_key = env::var("OPENWEATHER_API_KEY").unwrap();

    println!("{}", "Welcome to Weather Warp".bright_cyan());
    println!("Enter your city: ");
    let mut city = String::new();
    io::stdin()
        .read_line(&mut city)
        .expect("Failed to read input...");
    let city = city.trim();

    println!("Enter your country code (eg. IND for India): ");
    let mut country = String::new();
    io::stdin()
        .read_line(&mut country)
        .expect("Failed to read input...");
    let country = country.trim();

    match get_weather_info(&city, &country, &api_key) {
        Ok(response) => {
            display(&response);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}
