use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub date: String,
    pub temperature_c: f64,
    pub temperature_f: f64,
    pub summary: String,
    pub city : String,
}

impl Default for WeatherForecast {
    fn default() -> Self {
        Self {
            date: "2021-01-01".to_string(),
            temperature_c: 0.0,
            temperature_f: 32.0,
            summary: "Freezing".to_string(),
            city : "Seoul".to_string(),
        }
    }
}
