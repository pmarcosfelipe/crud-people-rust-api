use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::Serialize;
use time::{macros::date, Date};
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nickname: String,
    #[serde(with = "date_format")]
    pub birthdate: Date,
    pub stack: Vec<String>,
}

type AppState = Arc<HashMap<Uuid, Person>>;

#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person: Person = Person {
        id: Uuid::now_v7(),
        name: String::from("Marcos Felipe"),
        nickname: String::from("marcosvieira"),
        birthdate: date!(1992 - 04 - 12),
        stack: vec!["frontend".to_string(), "backend".to_string()],
    };

    people.insert(person.id, person);

    let app_state: AppState = Arc::new(people);

    // build our application with a single route
    let app = Router::new()
        .route("/people", get(get_people))
        .route("/people/:id", get(get_person_by_id))
        .route("/people", post(create_person))
        .route("/count-people", get(count_people))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Search People")
}

async fn get_person_by_id(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {
    match people.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Create Person!")
}

async fn count_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Count People!")
}
