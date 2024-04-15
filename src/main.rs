use std::{error::Error, io::{self, Write}};

use serde::Deserialize;
use reqwest;

#[derive(Deserialize)]
struct WeatherData {
    hourly: HourlyData,
    current: CurrentData,
}

#[derive(Deserialize)]
struct HourlyData {
    time: Vec<String>,
    temperature_2m: Vec<f32>,
    pressure_msl: Vec<f32>,
    surface_pressure: Vec<f32>,
}

#[derive(Deserialize)]
struct CurrentData {
    time: String,
    temperature_2m: f32,
    pressure_msl: f32,
    surface_pressure: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let latitude = 0;
    let longitude = 0;
    let current_weather_params = "temperature_2m,pressure_msl,surface_pressure";
    let hourly_weather_params = "temperature_2m,pressure_msl,surface_pressure";
    let timezone = "auto";
    let past_days = 2;
    let forecast_days = 3;
    let models = "icon_eu";

    let url = format!(
        "https://api.open-meteo.com/v1/dwd-icon?latitude={}&longitude={}&current={}&hourly={}&timezone={}&past_days={}&forecast_days={}&models={}",
        latitude, longitude, current_weather_params, hourly_weather_params, timezone, past_days, forecast_days, models
    );


    let response = reqwest::blocking::get(url)?;

    let text = response.text()?;

    let weather_data: WeatherData = serde_json::from_str(&text)?;

    println!("Past Weather Data:");
    // for every hourly weather data stored, print every sixth to minimize overall data printed out
    for (index, time) in weather_data.hourly.time.iter().enumerate().step_by(6) {
        println!("Time: {}", time);
        println!("Temperature: {}°C", weather_data.hourly.temperature_2m[index]);
        println!("Pressure MSL: {} hPa", weather_data.hourly.pressure_msl[index]);
        println!("Surface Pressure: {} hPa\n", weather_data.hourly.surface_pressure[index]);
    }

    println!("\n\n\nCurrent Weather Data:");
    println!("Time: {}", weather_data.current.time);
    println!("Temperature: {}°C", weather_data.current.temperature_2m);
    println!("Pressure MSL: {} hPa", weather_data.current.pressure_msl);
    println!("Surface Pressure: {} hPa", weather_data.current.surface_pressure);

    println!("\nPress Enter to exit...");
    io::stdout().flush()?;
    io::stdin().read_line(&mut String::new())?;

    Ok(())
}

