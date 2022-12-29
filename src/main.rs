// main.rs
// This is a bare-bones starter for an API using the Axum web framework
// it has a single /health_check route

// import dependencies
use axum::{http::StatusCode, response::Html, routing::get, Router};
use color_eyre::eyre::Result;
use futures::future::pending;
use std::net::SocketAddr;
use tokio::signal;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

// function to handle graceful shutdown on ctl-c
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl-C graceful shutdown handler");
    };

    // configuration for graceful shutdown on Unix platforms
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    // configuration for graceful shutdown on non-Unix platforms
    #[cfg(not(unix))]
    let terminate = pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn root() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::OK,
        Html("<h1>Welcome to the Axum Core API</h1><h2>Available routes:</h2><p>/ - this route, the root</p><p>/health_check - current API status</p>")
    )
}

// handler function for our "/health_check" route
async fn health_check() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::OK,
        Html("<h1>Welcome to the Axum Core API</h1><h2>Status:</h2><p>Alive, 200 OK</p>"),
    )
}

// handler function for any other route than root or /health_check
async fn not_found() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::NOT_FOUND,
        Html("<h1>Nothing here by that name...yet.</h1>"),
    )
}

// main application
#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    // routes for our core API application
    let app = Router::new()
        // root route
        .route("/", get(root))
        // health_check route
        .route("/health_check", get(health_check))
        .route("/not_found", get(not_found));

    // spin up and listen on port 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on port: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
