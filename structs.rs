use serde::{Deserialize,Serialize};




// Structs to represent the weather data response
#[derive(Deserialize,Serialize)]
pub struct WeatherResponse {
    pub name: String,
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize,Serialize)]
pub struct Main {
    pub temp: f64,
    pub humidity: u64,
}

#[derive(Deserialize,Serialize)]
pub struct Weather {
    pub description: String,
}
