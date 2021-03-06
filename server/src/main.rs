use axum::{
	response::IntoResponse,
	routing::{get, post},
	Json, Router,
	http::StatusCode, AddExtensionLayer, extract::{Extension, Query}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{net::SocketAddr};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer, Origin};

use serde::{Serialize, Deserialize};

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;



#[tokio::main]
async fn main() {
	// set up tracing
	tracing_subscriber::registry()
		.with(tracing_subscriber::EnvFilter::new(
			std::env::var("RUST_LOG")
				.unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
		))
		.with(tracing_subscriber::fmt::layer())
		.init();

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
		.route("/login", post(login))
		.route("/tasks", get(get_tasks).post(add_task))
		.layer(cors_layer)
		.layer(AddExtensionLayer::new(pool))
		.layer(TraceLayer::new_for_http());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    println!("listening on {}", addr);
	tracing::debug!("listening on {}", addr);
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

#[derive(Debug, Deserialize, Serialize)]
struct UserId {
	id: uuid::Uuid
}

type ConnPool = Pool<PostgresConnectionManager<NoTls>>;

async fn register(Json(payload): Json<InputUser>, Extension(pool): Extension<ConnPool> ) -> impl IntoResponse {
	dbg!("registering user");
	let person = create_person(payload, pool).await;

	match person {
		Some(person) => return (StatusCode::CREATED, Json(person)),
		None => return (StatusCode::INTERNAL_SERVER_ERROR, Json(User{login: String::from("fake"), id: uuid::Builder::nil().build()}))
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

async fn login(Json(payload): Json<InputUser>, Extension(pool): Extension<ConnPool>) -> impl IntoResponse {
	dbg!("user login", &payload);

	let person = authenticate(payload, pool).await;

	match person {
		Some(person) => return (StatusCode::OK, Json(person)),
		None => return (StatusCode::UNAUTHORIZED, Json(User{login: String::from("fake"), id: uuid::Builder::nil().build()}))
	}
}

async fn authenticate(person: InputUser, pool: ConnPool) -> Option<User> {
	let conn = pool.get().await;
	match &conn {
		Ok(_conn) => {}
		Err(_e) => return None
	}
	dbg!("got connection?");

	let row = conn.unwrap()
		.query_one(
			"select * from person where login = $1",
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

#[derive(Debug, Deserialize)]
struct InputTask {
	text: String,
	personid: uuid::Uuid
}

#[derive(Debug, Serialize)]
struct Task {
	id: i64,
	text: String,
	personid: uuid::Uuid
}

#[derive(Debug, Serialize)]
struct TaskList {
	tasks: Vec<Task>
}

impl TaskList {
	fn new() -> Self {
		Self {
			tasks: Vec::new()
		}
	}
}

async fn add_task(Json(payload): Json<InputTask>, Extension(pool): Extension<ConnPool>) -> impl IntoResponse {
	dbg!("add task", &payload);

	let task = create_task(payload, pool).await;

	match task {
		Some(task) => return (StatusCode::OK, Json(task)),
		None => return (StatusCode::UNAUTHORIZED, Json(Task{id: 0, text: String::from("fake"), personid: uuid::Builder::nil().build()}))
	}
}

async fn create_task(task: InputTask, pool: ConnPool) -> Option<Task> {
	let conn = pool.get().await;
	match &conn {
		Ok(_conn) => {}
		Err(_e) => return None
	}
	dbg!("got connection?");

	let row = conn.unwrap()
		.query_one(
			"insert into task (text, personid) values ($1, $2) returning id, text, personid",
			&[&task.text, &task.personid]
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

	let new_task = Task {
		id: urow.get("id"),
		text: urow.get("text"),
		personid: urow.get("personid")
	};

	Some(new_task)
}



async fn get_tasks(Query(userid): Query<UserId>, Extension(pool): Extension<ConnPool>) -> impl IntoResponse {
	// dbg!("get tasks", &uid);

	let tasks = query_tasks(userid, pool).await;

	match tasks {
		Some(tasks) => return (StatusCode::OK, Json(tasks)),
		None => return (StatusCode::UNAUTHORIZED, Json(TaskList::new()))
	}
}

async fn query_tasks(userid: UserId, pool: ConnPool) -> Option<TaskList> {
	let conn = pool.get().await;
	match &conn {
		Ok(_conn) => {}
		Err(_e) => return None
	}
	dbg!("got connection?");

	let rows = conn.unwrap()
		.query(
			"select * from task where personid = $1",
			&[&userid.id]
		)
		.await;

	match &rows {
		Ok(_rows) => {}
		Err(e) => {
			dbg!(e);
			return None}
	}
	dbg!("got row?");

	let mut list = TaskList::new();

	for row in rows.unwrap() {
		list.tasks.push(Task {
			id: row.get("id"),
			text: row.get("text"),
			personid: row.get("personid")
		});
	}

	

	Some(list)
}