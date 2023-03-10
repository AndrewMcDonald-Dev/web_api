use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use config::{Config, ConfigError, Environment};
use deadpool_postgres::{Config as DPConfig, Pool};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::row::Row;
use uuid::Uuid;

// #[tokio::main(worker_threads=10)]
#[tokio::main(worker_threads = 1)]
async fn main() {
    dotenv().ok();
    let app_state = AppState::create().await;
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/persons", get(get_all).post(post_person))
        .route("/persons/:id", get(get_by_id))
        .with_state(Arc::clone(&app_state));
    println!("Ready and listening on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct ConfigBuilder {
    postgres: DPConfig,
}

impl ConfigBuilder {
    async fn from_env() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(
                Environment::with_prefix("POSTGRES")
                    .separator("_")
                    .keep_prefix(true)
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
    }
}

struct AppState {
    pool: Pool,
}

impl AppState {
    async fn create() -> Arc<AppState> {
        let cfg_builder = ConfigBuilder::from_env().await.unwrap();
        let pool = cfg_builder
            .postgres
            .create_pool(
                Some(deadpool_postgres::Runtime::Tokio1),
                tokio_postgres::NoTls,
            )
            .unwrap();
        Arc::new(AppState { pool })
    }
}

async fn get_all(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = state
        .pool
        .get()
        .await
        .expect("Could not obtain pooled connection.");
    let rows = client
        .query("SELECT id, first_name, last_name FROM person", &[])
        .await
        .unwrap();
    let mut persons: Vec<Person> = Vec::new();
    for row in rows.iter() {
        persons.push(row.into());
    }
    (StatusCode::OK, Json(persons))
}

async fn get_by_id(Path(id): Path<Uuid>, State(state): State<Arc<AppState>>) -> Response {
    let client = state
        .pool
        .get()
        .await
        .expect("Could not obtain pooled connection.");
    let rows = client
        .query(
            "SELECT id, first_name, last_name FROM person WHERE id=$1",
            &[&id],
        )
        .await
        .unwrap();
    match rows.first() {
        Some(row) => {
            let person: Person = row.into();
            (StatusCode::OK, Json(person)).into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Deserialize)]
struct CreatePerson {
    first_name: String,
    last_name: String,
}

async fn post_person(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePerson>,
) -> Response {
    let client = state
        .pool
        .get()
        .await
        .expect("Could not obtain pooled connection.");

    let new_person = Person {
        id: Uuid::new_v4(),
        first_name: payload.first_name,
        last_name: payload.last_name,
    };

    match client
        .query(
            "INSERT INTO person (id, first_name, last_name) VALUES ($1, $2, $3);",
            &[
                &new_person.id.to_string(),
                &new_person.first_name,
                &new_person.last_name,
            ],
        )
        .await
    {
        Ok(_) => (StatusCode::OK, Json(new_person)).into_response(),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, err.to_string()).into_response(),
    }
}

#[derive(Serialize)]
struct Person {
    id: Uuid,
    first_name: String,
    last_name: String,
}

impl From<&Row> for Person {
    fn from(row: &Row) -> Self {
        let id: Uuid = Uuid::try_parse(row.get("id")).expect("Bad id received.");
        let first_name: String = row.get("first_name");
        let last_name: String = row.get("last_name");
        Person {
            id,
            first_name,
            last_name,
        }
    }
}
