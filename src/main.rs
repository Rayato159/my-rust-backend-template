use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    error_handling::HandleErrorLayer,
    http::{Method, StatusCode},
    routing::post,
    BoxError, Router,
};
use rust_backend_template::{
    database::postgres,
    handlers::{
        not_found::not_found,
        users::{registration, UsersHandler},
    },
    setting::app::Setting,
};
use tokio::{net::TcpListener, signal};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
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

    let users_handler = UsersHandler::new(db_pool.clone());

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
        .layer(RequestBodyLimitLayer::new(
            (setting.server.body_limit * 1024 * 1024)
                .try_into()
                .unwrap(),
        ))
        .route(
            "/users",
            post({
                let users_service = Arc::clone(&users_handler.users_service);
                move |req| registration(req, users_service)
            }),
        )
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(
                    setting.server.timeout.try_into().unwrap(),
                ))),
        )
        .fallback(not_found);

    let addr = SocketAddr::from(([0, 0, 0, 0], setting.server.port as u16));

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server running on port {}", setting.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Starting graceful shutdown");
        },
        _ = terminate => {},
    }
}
