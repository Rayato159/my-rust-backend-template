use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Path, http::Method, routing::get, Json, Router};
use rust_backend_template::{
    database::postgres,
    models::{
        error::{CustomError, ErrorResponse},
        user::{User, UserNotFoundError},
    },
    setting::app::Setting,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Setting::new().unwrap();
    info!("setting has been loaded.");

    let db_pool = postgres::conn_getting(Arc::clone(&setting)).await.unwrap();
    info!("database connection has been established.");

    // build our application with a single route
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .route("/users/:id", get(get_user))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], setting.server.port as u16));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server running on port {}", setting.server.port);
    axum::serve(listener, app).await.unwrap();
}

async fn get_user(Path(user_id): Path<i64>) -> Result<Json<User>, Json<ErrorResponse>> {
    if user_id == 1 {
        return Ok(Json(User {
            id: 1,
            username: "john_doe".to_string(),
            password: "123456".to_string(),
        }));
    }
    Err(Json(UserNotFoundError::new(user_id).error()))
}
