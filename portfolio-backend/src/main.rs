use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::main;
use tower_http::cors::{CorsLayer, Any};

mod models;
mod routes;

#[main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/projects", axum::routing::get(routes::get_projects))
        .layer(cors);
    
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .unwrap_or(3001);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

