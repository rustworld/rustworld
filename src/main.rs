use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("Start Axum Web Server");
    // build our application with a route
    let app = app();

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app () -> Router {
    let static_files = ServeDir::new("./static");

    Router::new()
        .route("/hello", get(|| async {"hello"}))
        .fallback_service(static_files)
}