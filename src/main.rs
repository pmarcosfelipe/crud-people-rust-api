use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::option::Option;
use std::{collections::HashMap, sync::Arc};
use time::{macros::date, Date};
use tokio::sync::RwLock;
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nickname: String,
    #[serde(with = "date_format")]
    pub birthdate: Date,
    pub stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    pub name: String,
    pub nickname: String,
    #[serde(with = "date_format")]
    pub birthdate: Date,
    pub stack: Option<Vec<String>>,
}

type AppState = Arc<RwLock<HashMap<Uuid, Person>>>;

#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person: Person = Person {
        id: Uuid::now_v7(),
        name: String::from("Marcos Felipe"),
        nickname: String::from("marcosvieira"),
        birthdate: date!(1992 - 04 - 12),
        stack: vec!["frontend".to_string(), "backend".to_string()].into(),
    };

    println!("{}", person.id);

    people.insert(person.id, person);

    let app_state: AppState = Arc::new(RwLock::new(people));

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
    match people.read().await.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse {
    if new_person.name.len() > 100 || new_person.nickname.len() > 32 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let id = Uuid::now_v7();

    let person: Person = Person {
        id,
        name: new_person.name,
        nickname: new_person.nickname,
        birthdate: new_person.birthdate,
        stack: new_person.stack,
    };

    people.write().await.insert(id, person.clone());

    Ok((StatusCode::OK, Json(person)))
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    let count = people.read().await.len();
    (StatusCode::NOT_FOUND, Json(count));
}
