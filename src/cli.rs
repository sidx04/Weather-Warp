use colored::*;
use core::result::Result;
use serde::Deserialize;

/// all the structs are made corresponding to the JSON response
/// JSON response:
///                           
/// {
///     "coord": {
///       "lon": 10.99,
///       "lat": 44.34
///     },
///     "weather": [
///       {
///         "id": 501,
///         "main": "Rain",
///         "description": "moderate rain",
///         "icon": "10d"
///       }
///     ],
///     "base": "stations",
///     "main": {
///       "temp": 298.48,
///       "feels_like": 298.74,
///       "temp_min": 297.56,
///       "temp_max": 300.05,
///       "pressure": 1015,
///       "humidity": 64,
///       "sea_level": 1015,
///       "grnd_level": 933
///     },
///     "visibility": 10000,
///     "wind": {
///       "speed": 0.62,
///       "deg": 349,
///       "gust": 1.18
///     },
///     "rain": {
///       "1h": 3.16
///     },
///     "clouds": {
///       "all": 100
///     },
///     "dt": 1661870592,
///     "sys": {
///       "type": 2,
///       "id": 2075663,
///       "country": "IT",
///       "sunrise": 1661834187,
///       "sunset": 1661882248
///     },
///     "timezone": 7200,
///     "id": 3163858,
///     "name": "Zocca",
///     "cod": 200
///   }

// struct to deserialise JSON response from openweather API
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// struct to represent weather desc
#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

// struct to represent main weather parameters
#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    humidity: f64,
    pressure: f64,
}

// struct to represent wind information
#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}

// function to get weather information from API
pub fn get_weather_info(
    city: &str,
    country_code: &str,
    api_url: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "{}/?q={},{}&units=metric&appid={}",
        api_url, city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url).unwrap();
    let response_json = response.json::<WeatherResponse>().unwrap();
    Ok(response_json)
}

//  function to display weather information:

pub fn display(res: &WeatherResponse) -> String {
    // extract weather information from response
    let description = &res.weather[0].description;
    let temp = &res.main.temp;
    let feels_like = &res.main.feels_like;
    let humidity = &res.main.humidity;
    let pressure = &res.main.pressure;
    let wind_speed = &res.wind.speed;

    let weather_text = format!(
        "Weather in {}: {}, {}
        > Temperature: {:.1}°C      [Feels like: {:.1}°C]
        > Humidity: {:.1}%
        > Pressure: {:.2}hPa
        > Wind Speed: {:.1} m/s",
        res.name,
        description,
        generate_emote(*temp),
        temp,
        feels_like,
        humidity,
        pressure,
        wind_speed
    );

    let x = color_weather_text(&description, &weather_text);
    println!("{}", x);
    x.to_string()
}

pub fn color_weather_text(description: &str, weather_text: &str) -> ColoredString {
    let weather_text_colored = match description {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    weather_text_colored
}

pub fn generate_emote(temp: f64) -> &'static str {
    match temp {
        t if t < 0.0 => "❄️",
        t if t < 10.0 => "☁️",
        t if t < 20.0 => "⛅",
        t if t < 30.0 => "🌤️",
        _ => "🔥",
    }
}
