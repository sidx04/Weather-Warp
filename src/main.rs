use dotenv::dotenv;
use std::fs;

use configure::DefaultConfig;

mod cli;

mod args;

mod configure;

fn main() {
    dotenv().ok();

    let args = args::Args::arg_parse();

    // If no command is provided, execute the `Weather` command by default
    let command = match args.command {
        Some(c) => c,
        None => {
            let file = fs::read_to_string("./default.json").expect("Unable to read file...");
            let default_data: DefaultConfig = serde_json::from_str(&file).unwrap();
            println!("{:#?}", default_data);
            args::Command::Weather {
                city: Some(default_data.city.to_string()),
                country: Some(default_data.country.to_string()),
            }
        }
    };

    match command {
        args::Command::Weather { city, country } => {
            cli::run(city, country);
        }
        args::Command::Config { city, country } => {
            let _config_string = configure::config(city.unwrap(), country.unwrap());
        }
    }
}
