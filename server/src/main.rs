use axum::{
	response::{Html, IntoResponse},
	routing::{get, post},
	Json, Router,
	http::StatusCode};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer, Origin};

use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
	// create cors layer
	let cors_layer = CorsLayer::new()
		.allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
		.allow_methods(Any)
		.allow_headers(Any);

    // build our application with a route
    let app = Router::new()
		.route("/hello", get(say_hello))
		.route("/register", post(register))
		.layer(cors_layer);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello() -> impl IntoResponse {
	dbg!("Saying hello");
    (StatusCode::OK, Json(String::from("Hello, World!")))
}

#[derive(Debug, Deserialize)]
struct InputUser {
	login: String
}

async fn register(Json(payload): Json<InputUser>) -> impl IntoResponse {
	dbg!(payload);
	(StatusCode::CREATED, Json("hey, i made da user"))
}