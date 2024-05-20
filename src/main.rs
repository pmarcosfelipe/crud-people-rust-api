use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use time::{macros::date, Date};
use uuid::Uuid;

pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub nickname: String,
    pub birthdate: Date,
    pub stack: Vec<String>,
}

#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: String::from("Marcos Felipe"),
        nickname: String::from("marcosvieira"),
        birthdate: date!(1992 - 04 - 12),
        stack: vec!["frontend".to_string(), "backend".to_string()],
    };

    people.insert(person.id, person);

    // build our application with a single route
    let app = Router::new()
        .route("/people", get(get_people))
        .route("/people/:id", get(get_person_by_id))
        .route("/people", post(create_person))
        .route("/count-people", get(count_people));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Search People")
}

async fn get_person_by_id() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Get Person by Id!")
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Create Person!")
}

async fn count_people() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Count People!")
}
