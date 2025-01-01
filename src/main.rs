use {
    axum::Router,
    controllers::service_controllers,
    hyper::Method,
    log::info,
    std::env,
    tokio::signal,
    tower_http::cors::{Any, CorsLayer},
};

mod controllers;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("🚀 Server starting...");

    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or("development".to_string());
    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("80".to_string());

    info!(
        "Server configured to accept connections on host {}...",
        app_host
    );
    info!(
        "Server configured to listen connections on port {}...",
        app_port
    );

    let app_addr = app_host + ":" + &app_port;

    match app_environment.as_str() {
        "development" => {
            info!("Running in development mode");
        }
        "production" => {
            info!("Running in production mode");
        }
        _ => {
            info!("Running in development mode");
        }
    }

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let routes = Router::new()
        .merge(service_controllers::router())
        .layer(cors);

    axum::Server::bind(&app_addr.parse().expect("Valid address expected"))
        .serve(routes.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    info!("Server stopped");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
