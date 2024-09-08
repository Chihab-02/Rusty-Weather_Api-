use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
mod structs;
use structs::WeatherResponse;





// Function to fetch weather data from the API
async fn fetch_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
    let api_key = env::var("API_KEY").expect("API key not set");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<WeatherResponse>().await?;
    Ok(weather_data)
}

// Weather endpoint handler
async fn get_weather(city: web::Path<String>) -> impl Responder {
    match fetch_weather(&city).await {
        Ok(weather_data) => HttpResponse::Ok().json(weather_data),
        Err(_) => HttpResponse::InternalServerError().body("Could not fetch weather data"),
    }
}

// Main function to run the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from the .env file
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .route("/weather/{city}", web::get().to(get_weather)) // Set up route
    })
    .bind("127.0.0.1:8080")? // Bind server to port 8080
    .run()
    .await
}

