use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

// {
//      "coord": {
//        "lon": -0.13,
//        "lat": 51.51
//      },
//      "weather": [
//        {
//          "id": 300,
//          "main": "Drizzle",
//          "description": "light intensity drizzle",
//          "icon": "09d"
//        }
//      ],
//      "base": "stations",
//      "main": {
//        "temp": 280.32,
//        "pressure": 1012,
//        "humidity": 81,
//        "temp_min": 279.15,
//        "temp_max": 281.15
//      },
//      "visibility": 10000,
//      "wind": {
//        "speed": 4.1,
//        "deg": 80
//      },
//      "clouds": {
//        "all": 90
//      },
//      "dt": 1485789600,
//      "sys": {
//        "type": 1,
//        "id": 5091,
//        "message": 0.0103,
//        "country": "GB",
//        "sunrise": 1485762037,
//        "sunset": 1485794875
//      },
//      "id": 2643743,
//      "name": "London",
//      "cod": 200
//      }

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: u64,
    pub sys: Sys,
    pub id: u64,
    pub name: String,
    pub cod: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub temp: Option<f64>,
    pub pressure: u32,
    pub humidity: u8,
    pub temp_min: f64,
    pub temp_max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clouds {
    pub all: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub sys_type: u8,
    pub id: u32,
    pub message: Option<f64>,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key: String = env::var("RUSTY_WEATHER_API_KEY")
        .expect("Please set the RUSTY_WEATHER_API_KEY  environment variable!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: weather_cli <city>");
        std::process::exit(1);
    }

    let city: &String = &args[1];

    let client = Client::new();
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );

    let weather_data: WeatherResponse = client.get(&url).send()?.json()?;

    if let Some(temp) = weather_data.main.temp {
        let celsius = temp - 273.15;
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        println!(
            "Temperature: {:.2}C / {:.2}F ",
            celsius,
            fahrenheit
        )
    } else {
        println!("main not defined")
    }

    // print!("DEBUG! :: city value: {}", city);
    // print!("weather_data: {}", weather_data);
    Ok(())
}
