use std::env;

use colored::*;
// use figment::{
//     providers::{Format, Toml},
//     Figment,
// };
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub struct DefaultConfig {
    pub city: String,
    pub country: String,
}

pub fn config(city: String, country: String) {
    let mut msg;
    msg = format!(
        "Configuring {} and {} to default...",
        city.red(),
        country.red()
    );
    println!("{}", msg.bright_yellow());

    let args: Vec<String> = env::args().collect();

    let types = &args[1];
    let country = &args[2];

    println!("{}, {}", types, country);

    msg = format!("Config set.");
    println!("{}", msg.bright_blue());
}
