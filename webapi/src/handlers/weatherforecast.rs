use axum::extract::Path;
// use axum::http::StatusCode;
use axum::{routing::get, Router};
use axum::response::{IntoResponse, Json};

use crate::models::weatherforecast::WeatherForecast;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_weather_forecasts, get_weather_forecast_by_city),
    components(schemas(WeatherForecast)),
)]
pub struct WeatherForecastOpenApi;

pub fn router() -> Router {
    Router::new()
        .route("/weatherforecast", get(get_weather_forecasts))
        .route("/weatherforecast/*city", get(get_weather_forecast_by_city))
}

#[utoipa::path(
    get, 
    tag = stringify!(WeatherForecast), 
    path = "/weatherforecast", 
    operation_id = stringify!(get_weather_forecasts),
    responses(
        (
            status = OK, 
            body = Vec<WeatherForecast>, 
            description = "Get weather forecasts",
            content_type = "application/json",
        )
    )
)]
pub async fn get_weather_forecasts() -> impl IntoResponse {
    let mut weather_forecasts = Vec::new();
    for _ in 0..5 {
        weather_forecasts.push(WeatherForecast {
            date: "2021-01-01".to_string(),
            temperature_c: 0.0,
            temperature_f: 32.0,
            summary: "Freezing".to_string(),
            ..Default::default()
        });
    }
    Json(weather_forecasts)
}

#[utoipa::path(
    get, 
    tag = stringify!(WeatherForecast), 
    path = "/weatherforecast/{city}", 
    operation_id = stringify!(get_weather_forecast_by_city),
    // params(
    //     ("city" = &str, Path, description = "City name"),
    // ),
    responses(
        (
            status = OK, 
            body = WeatherForecast, 
            description = "Get weather forecast",
            content_type = "application/json",
        ),
        (
            status = NOT_FOUND, 
            description = "City not found",
        )
    )
)]
pub async fn get_weather_forecast_by_city(Path(city) : Path<String>) -> impl IntoResponse {
    let weather_forecast = WeatherForecast {
        date: "2021-01-01".to_string(),
        temperature_c: 0.0,
        temperature_f: 32.0,
        summary: "Freezing".to_string(),
        city: city.to_string(),
    };
    // StatusCode::NOT_FOUND
    Json(weather_forecast)
}
