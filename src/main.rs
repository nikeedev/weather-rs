use serde::Deserialize;
use reqwest::header::{CONTENT_LOCATION, CONTENT_TYPE, USER_AGENT};

#[derive(Debug, Deserialize, Clone)]
struct IPGeo {
    status: String,
    continent: Option<String>,
    country: Option<String>,
    countryCode: Option<String>,
    regionName: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    timezone: Option<String>,
    currency: Option<String>,
    isp: Option<String>,
    org: Option<String>,
    #[serde(rename = "as")]
    as_server: Option<String>,
    reverse: Option<String>,
    mobile: Option<bool>,
    proxy: Option<bool>,
    ip: Option<String>,
    cached: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let user_info = reqwest::get("https://api.techniknews.net/ipgeo/")
        .await?
        .text()
        .await?;

    let user_info: IPGeo = serde_json::from_str(&user_info).unwrap();

    let lat = user_info.lat.unwrap();
    let long = user_info.lon.unwrap();

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let nowcast = client
        .get(format!(
            "https://api.met.no/weatherapi/nowcast/2.0/complete?lat={}&lon={}",
            lat, long
        ))
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "weather-rs 1.0/nikeedev")
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", nowcast);

    Ok(())
}
