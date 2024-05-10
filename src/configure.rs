use colored::*;
use config::Config;
use std::collections::HashMap;
use std::fs::{remove_file, OpenOptions};
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn config(city: String, country: String) {
    let mut msg;

    // Colored message highlighting the city and country being configured
    msg = format!(
        "Configuring {} and {} to default...",
        city.red(),
        country.red()
    );
    println!("{}", msg.bright_yellow());

    // Create a shared HashMap for storing the configuration data
    let config_data = Arc::new(Mutex::new(HashMap::new()));
    let config_data_clone = Arc::clone(&config_data); // Clone the Arc

    // Thread to load or create the default configuration
    let load_config_thread = thread::spawn(move || {
        // Remove the existing JSON file (if it exists)
        remove_file("./default.json")
            .unwrap_or_else(|_| println!("File doesn't exist, ignoring removal..."));

        let json_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./default.json")
            .unwrap();

        let _json = BufWriter::new(json_file);

        // Load the configuration from the JSON file (if it exists)
        let default = Config::builder()
            .add_source(config::File::with_name("./default.json"))
            .build()
            .unwrap();

        let default_config = default
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

        // Access the HashMap using the moved config_data
        let mut config_data_lock = config_data_clone.lock().unwrap();
        *config_data_lock = default_config;
    });

    // Wait for the loading thread to finish
    let _ = load_config_thread.join();

    // Access and update the configuration data from the shared HashMap
    let mut config_data_lock = config_data.lock().unwrap();
    config_data_lock.insert("city".to_string(), city.clone());
    config_data_lock.insert("country".to_string(), country.clone());

    // Serialize the updated HashMap back into JSON format
    let serialized_config = serde_json::to_string(&*config_data_lock).unwrap();

    // Open the file for writing
    let json_file = OpenOptions::new()
        .write(true)
        .create(true) // Overwrite if the file already exists
        .open("./default.json")
        .unwrap();

    let mut json = BufWriter::new(json_file);

    let serialized_config = serialized_config.trim_end_matches('\n').to_string();
    json.write_all(serialized_config.as_bytes()).unwrap();

    println!("{:#?}", serialized_config);

    msg = format!("Config set.");
    println!("{}", msg.bright_blue());
}
