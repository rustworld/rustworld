mod models;

use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tower_http::services::ServeDir;
use models::AppState;
use crate::models::url_map::{CreateItem, UpdateItem};

#[tokio::main]
async fn main() {
    println!("Start Axum Web Server");
    // build our application with a route
    let app = app();

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app () -> Router{
    let static_files = ServeDir::new("./static");
    let shared_state = AppState::new();

    Router::new()
        .route("/info",            get(|State(s): State<AppState>|                                   async move {s.get_info()}))
        .route("/api/urls/{key}",  delete(|State(s): State<AppState>, Path(key): Path<u32>|          async move {s.urls.delete(key)}).
                                   put(|State(s): State<AppState>, Path(key): Path<u32>, Json(payload): Json<UpdateItem>| async move {s.urls.update(key, payload)}).
                                   get(|State(s): State<AppState>, Path(key): Path<u32>|             async move {s.urls.read(key)}))
        .route("/api/urls",        post(|State(s): State<AppState>, Json(payload): Json<CreateItem>| async move {s.urls.create(payload)}).
                                   get(|State(s): State<AppState>|                                   async move {s.urls.all()}))
        .with_state(shared_state)
        .fallback_service(static_files)
}