use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde_json::Value;
use std::env;
use chrono::{DateTime, Local, TimeZone, Utc};

fn direction<'a>(degrees: f64) -> &'a str {
    match degrees {
        0.0..22.5 => "↑",
        22.6..67.5 => "↗",
        67.6..112.5 => "→",
        112.6..157.5 => "↘",
        157.6..202.5 => "↓",
        202.6..247.5 => "↙",
        247.6..292.5 => "←",
        292.6..337.5 => "↖",
        337.5..360.0 => "↑",
        _ => "NaN"
    }
} 


fn weather_description(value_name: &str) -> &str {
    match value_name {
        "clearsky_day" => "Clear sky (day)",
        "clearsky_night" => "Clear sky (night)",
        "clearsky_polartwilight" => "Clear sky (polar twilight)",
        "fair_day" => "Fair (day)",
        "fair_night" => "Fair (night)",
        "fair_polartwilight" => "Fair (polar twilight)",
        "partlycloudy_day" => "Partly cloudy (day)",
        "partlycloudy_night" => "Partly cloudy (night)",
        "partlycloudy_polartwilight" => "Partly cloudy (polar twilight)",
        "cloudy" => "Cloudy",
        "rainshowers_day" => "Rain showers (day)",
        "rainshowers_night" => "Rain showers (night)",
        "rainshowers_polartwilight" => "Rain showers (polar twilight)",
        "rainshowersandthunder_day" => "Rain showers and thunder (day)",
        "rainshowersandthunder_night" => "Rain showers and thunder (night)",
        "rainshowersandthunder_polartwilight" => "Rain showers and thunder (polar twilight)",
        "sleetshowers_day" => "Sleet showers (day)",
        "sleetshowers_night" => "Sleet showers (night)",
        "sleetshowers_polartwilight" => "Sleet showers (polar twilight)",
        "snowshowers_day" => "Snow showers (day)",
        "snowshowers_night" => "Snow showers (night)",
        "snowshowers_polartwilight" => "Snow showers (polar twilight)",
        "rain" => "Rain",
        "heavyrain" => "Heavy rain",
        "heavyrainandthunder" => "Heavy rain and thunder",
        "sleet" => "Sleet",
        "snow" => "Snow",
        "snowandthunder" => "Snow and thunder",
        "fog" => "Fog",
        "sleetshowersandthunder_day" => "Sleet showers and thunder (day)",
        "sleetshowersandthunder_night" => "Sleet showers and thunder (night)",
        "sleetshowersandthunder_polartwilight" => "Sleet showers and thunder (polar twilight)",
        "snowshowersandthunder_day" => "Snow showers and thunder (day)",
        "snowshowersandthunder_night" => "Snow showers and thunder (night)",
        "snowshowersandthunder_polartwilight" => "Snow showers and thunder (polar twilight)",
        "rainandthunder" => "Rain and thunder",
        "sleetandthunder" => "Sleet and thunder",
        "lightrainshowersandthunder_day" => "Light rain showers and thunder (day)",
        "lightrainshowersandthunder_night" => "Light rain showers and thunder (night)",
        "lightrainshowersandthunder_polartwilight" => "Light rain showers and thunder (polar twilight)",
        "heavyrainshowersandthunder_day" => "Heavy rain showers and thunder (day)",
        "heavyrainshowersandthunder_night" => "Heavy rain showers and thunder (night)",
        "heavyrainshowersandthunder_polartwilight" => "Heavy rain showers and thunder (polar twilight)",
        "lightssleetshowersandthunder_day" => "Light sleet showers and thunder (day)",
        "lightssleetshowersandthunder_night" => "Light sleet showers and thunder (night)",
        "lightssleetshowersandthunder_polartwilight" => "Light sleet showers and thunder (polar twilight)",
        "heavysleetshowersandthunder_day" => "Heavy sleet showers and thunder (day)",
        "heavysleetshowersandthunder_night" => "Heavy sleet showers and thunder (night)",
        "heavysleetshowersandthunder_polartwilight" => "Heavy sleet showers and thunder (polar twilight)",
        "lightssnowshowersandthunder_day" => "Light snow showers and thunder (day)",
        "lightssnowshowersandthunder_night" => "Light snow showers and thunder (night)",
        "lightssnowshowersandthunder_polartwilight" => "Light snow showers and thunder (polar twilight)",
        "heavysnowshowersandthunder_day" => "Heavy snow showers and thunder (day)",
        "heavysnowshowersandthunder_night" => "Heavy snow showers and thunder (night)",
        "heavysnowshowersandthunder_polartwilight" => "Heavy snow showers and thunder (polar twilight)",
        "lightrainandthunder" => "Light rain and thunder",
        "lightsleetandthunder" => "Light sleet and thunder",
        "heavysleetandthunder" => "Heavy sleet and thunder",
        "lightsnowandthunder" => "Light snow and thunder",
        "heavysnowandthunder" => "Heavy snow and thunder",
        "lightrainshowers_day" => "Light rain showers (day)",
        "lightrainshowers_night" => "Light rain showers (night)",
        "lightrainshowers_polartwilight" => "Light rain showers (polar twilight)",
        "heavyrainshowers_day" => "Heavy rain showers (day)",
        "heavyrainshowers_night" => "Heavy rain showers (night)",
        "heavyrainshowers_polartwilight" => "Heavy rain showers (polar twilight)",
        "lightsleetshowers_day" => "Light sleet showers (day)",
        "lightsleetshowers_night" => "Light sleet showers (night)",
        "lightsleetshowers_polartwilight" => "Light sleet showers (polar twilight)",
        "heavysleetshowers_day" => "Heavy sleet showers (day)",
        "heavysleetshowers_night" => "Heavy sleet showers (night)",
        "heavysleetshowers_polartwilight" => "Heavy sleet showers (polar twilight)",
        "lightsnowshowers_day" => "Light snow showers (day)",
        "lightsnowshowers_night" => "Light snow showers (night)",
        "lightsnowshowers_polartwilight" => "Light snow showers (polar twilight)",
        "heavysnowshowers_day" => "Heavy snow showers (day)",
        "heavysnowshowers_night" => "Heavy snow showers (night)",
        "heavysnowshowers_polartwilight" => "Heavy snow showers (polar twilight)",
        "lightrain" => "Light rain",
        "lightsleet" => "Light sleet",
        "heavysleet" => "Heavy sleet",
        "lightsnow" => "Light snow",
        "heavysnow" => "Heavy snow",
        _ => "Unknown weather condition",
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();

    // let user_info = reqwest::get("https://api.techniknews.net/ipgeo/")
    //     .await?
    //     .text()
    //     .await?;

    // let user_info: Value = serde_json::from_str(&user_info).unwrap();

    // let lat = user_info["lat"].as_f64().unwrap();
    // let long = user_info["lon"].as_f64().unwrap();
    
    // Nesheim
    let lat = 59.46279;
    let long = 5.57334;

    // println!("Getting weather for following location: {}, {}", lat, long);
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let locast = client
        .get(format!(
            "https://api.met.no/weatherapi/locationforecast/2.0/complete?lat={lat}&lon={long}&altitude=76"
        ))
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "weather 1.0/nikeedev")
        .send()
        .await?
        .text()
        .await?;

    let locast: Value = serde_json::from_str(&locast.as_str()).unwrap();

    let current_weather = locast["properties"]["timeseries"][0].clone();
    
    // println!("{}", current_weather["time"]);
    let weather_time = current_weather["time"].clone()
        .as_str()
        .and_then(|time_str| time_str.parse::<DateTime<Utc>>().ok())
        .map(|utc_time| utc_time.with_timezone(&Local).format("%d.%m.%Y %H:%M").to_string())
        .unwrap_or_else(|| "Invalid time".to_string());
    
    let current_weather = locast["properties"]["timeseries"][0]["data"].clone();

    
    match args.contains(&"--short".to_string()) {
        true => {
            println!("Weather at Nesheim");
            let now = current_weather["instant"]["details"].clone();
            println!("Temperature 🌡️: {}°C", now["air_temperature"]);
            println!("Wind 🌬️ : \n\tDirection {} ({}°) \n\tWind speed: {} m/s", direction(now["wind_from_direction"].as_f64().unwrap()), now["wind_from_direction"].as_f64().unwrap(), now["wind_speed"]);
           println!("UV level (at clear sky) ☀️: {}", now["ultraviolet_index_clear_sky"].as_f64().unwrap().floor());
        },

        false => {
            println!("Weather at Nesheim");
            let now = current_weather["instant"]["details"].clone();
            println!("Now ({}):", weather_time);
            println!("\tTemperature 🌡️: {}°C", now["air_temperature"]);
            println!("\tWind 🌬️ : \n\t\tDirection {} ({}°) \n\t\tWind speed: {} m/s", direction(now["wind_from_direction"].as_f64().unwrap()), now["wind_from_direction"].as_f64().unwrap(), now["wind_speed"]);
            println!("\tUV level (at clear sky) ☀️: {}", now["ultraviolet_index_clear_sky"].as_f64().unwrap().floor());
            
            let next_hour = current_weather["next_1_hours"]["details"].clone();
            println!("\nWeather next hour:");
            println!("\tPrecipitation probability: {}%", next_hour["probability_of_precipitation"]);
            println!("\tPrecipitation amount: {} mm", next_hour["precipitation_amount"]);
            println!("\tPrecipitation amount (min-max): {} mm - {} mm", next_hour["precipitation_amount_min"], next_hour["precipitation_amount_max"]);
            let borrow = current_weather["next_1_hours"]["summary"].clone();
            let next_hour = borrow["symbol_code"].as_str().unwrap();
            println!("\tWeather summary: {}", weather_description(next_hour));

            let six_hour = current_weather["next_6_hours"]["details"].clone();
            println!("\nWeather for the next 6 hours:");
            println!("\tTemperature (min-max): {}-{} °C", six_hour["air_temperature_min"], six_hour["air_temperature_max"]);
            println!("\tPrecipitation probability: {}%", six_hour["probability_of_precipitation"]);
            println!("\tPrecipitation amount: {} mm", six_hour["precipitation_amount"]);
            println!("\tPrecipitation amount (min-max): {} mm - {} mm", six_hour["precipitation_amount_min"], six_hour["precipitation_amount_max"]);
            let borrow = current_weather["next_6_hours"]["summary"].clone();
            let six_hour = borrow["symbol_code"].as_str().unwrap();
            println!("\tWeather summary: {}", weather_description(six_hour));
            
            let twelvehour = current_weather["next_12_hours"]["details"].clone();
            println!("\nWeather for the next 12 hours:");
            println!("\tPrecipitation probability: {}%", twelvehour["probability_of_precipitation"]);
            let borrow = current_weather["next_12_hours"]["summary"].clone();
            let twelvehour = borrow["symbol_code"].as_str().unwrap();
            println!("\tWeather summary: {}", weather_description(twelvehour));
 
        }
        
    }
    
     

    Ok(())
}
