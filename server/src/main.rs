use axum::{
    extract::Query,
    routing::get,
    Json, Router,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize)]
struct PointsRequest {
    n: usize,
}

async fn generate_points(Query(params): Query<PointsRequest>) -> Json<Vec<Point>> {
    let mut rng = rand::rngs::ThreadRng::default();
    let points = (0..params.n)
        .map(|_| Point {
            x: rng.random::<f64>(),
            y: rng.random::<f64>(),
        })
        .collect();
    Json(points)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/points", get(generate_points))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}