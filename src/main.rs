mod controllers;
mod structs;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use controllers::{auth::*, common::*, image::*}; // , user::*
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::{fs, net::SocketAddr, path::Path, sync::Arc, time::Duration};
use structs::common::{AppConfig, DatabaseConfig};
use tokio::signal;
use tower_http::{
    add_extension::AddExtensionLayer, compression::CompressionLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_config = AppConfig::new();
    let db_config = DatabaseConfig::new();

    let mut client_options = ClientOptions::parse(db_config.uri).await.unwrap();
    client_options.connect_timeout = db_config.connect_timeout;
    client_options.min_pool_size = db_config.min_pool_size;
    client_options.max_pool_size = db_config.max_pool_size;
    client_options.compressors = db_config.compressors;

    let thumbnail_path = Path::new("./data/thumbnails");
    let image_path = Path::new("./data/images");
    let file_path = Path::new("./data/files");

    fs::create_dir_all(&thumbnail_path).expect("Thumbnails directory could not be created");
    fs::create_dir_all(&image_path).expect("Images directory could not be created");
    fs::create_dir_all(&file_path).expect("Files directory could not be created");

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_config.db_name);

    let app = Router::new()
        .route("/", get(root))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(get_me))
        .route("/image/:id", post(post_image))
        .layer(AddExtensionLayer::new(db))
        .layer(AddExtensionLayer::new(Arc::new(app_config)))
        .layer(TimeoutLayer::new(Duration::from_secs(15)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        // .layer(RequestBodyLimitLayer::new(15 * 1024 * 1024))
        .layer(DefaultBodyLimit::max(15 * 1024 * 1024))
        .fallback(fallback_handler)
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server started, listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
}
