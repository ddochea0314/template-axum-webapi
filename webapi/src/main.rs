mod handlers;
mod models;

use utoipa::{openapi::{Info, OpenApiBuilder}, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use axum::Router;

use handlers::weatherforecast::{router, WeatherForecastOpenApi};

#[tokio::main]
async fn main() {
    let mut openapi = OpenApiBuilder::default()
    .info(Info::new("{{project-name}} API", "1.0.0"))
    .build();

    openapi.merge(WeatherForecastOpenApi::openapi());

    let app = Router::new()
    .merge(SwaggerUi::new("/swagger").url("/swagger.json", openapi))
    .merge(router());

    let tcp_listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
