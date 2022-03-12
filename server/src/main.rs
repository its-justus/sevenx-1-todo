use axum::{
	response::IntoResponse,
	routing::{get, post},
	Json, Router,
	http::StatusCode, AddExtensionLayer, extract::Extension};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer, Origin};

use serde::{Serialize, Deserialize};

use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
	// create connection pool
	let connman = PostgresConnectionManager::new_from_stringlike("host=localhost dbname=sevenx_1_todo user=sevenx_1_todo password=sevenx", NoTls).unwrap();
	let pool = Pool::builder().build(connman).await.unwrap();

	// create cors layer
	let cors_layer = CorsLayer::new()
		.allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
		.allow_methods(Any)
		.allow_headers(Any);

    // build our application with a route
    let app = Router::new()
		.route("/hello", get(say_hello))
		.route("/register", post(register))
		.layer(cors_layer)
		.layer(AddExtensionLayer::new(pool));

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

#[derive(Debug, Serialize)]
struct User {
	id: uuid::Uuid,
	login: String
}

type ConnPool = Pool<PostgresConnectionManager<NoTls>>;

async fn register(Json(payload): Json<InputUser>, Extension(pool): Extension<ConnPool> ) -> impl IntoResponse {
	dbg!("registering user");
	let person = create_person(payload, pool).await;

	match person {
		Some(person) => return (StatusCode::CREATED, Json(person)),
		None => return (StatusCode::CREATED, Json(User{login: String::from("fake"), id: uuid::Builder::nil().build()}))
	}
}

async fn create_person(person: InputUser, pool: ConnPool ) -> Option<User> {
	//TODO: fix this so we're not using unwrap. this function should be returning a result instead of an option
	dbg!("creating person");

	let conn = pool.get().await;
	match &conn {
		Ok(_conn) => {}
		Err(_e) => return None
	}
	dbg!("got connection?");

	let row = conn.unwrap()
		.query_one(
			"insert into person (login) values ($1) returning id, login",
			&[&person.login]
		)
		.await;

	match &row {
		Ok(_row) => {}
		Err(e) => {
			dbg!(e);
			return None}
	}
	dbg!("got row?");

	let urow = row.unwrap();

	let user = User {
		id: urow.get("id"),
		login: urow.get("login")
	};

	Some(user)
}