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
#[serde(try_from = "String")]
pub struct Name(String);

impl TryFrom<String> for Name {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 100 {
            Ok(Self(value))
        } else {
            Err("Name should contain less than 100 characters!")
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct Nickname(String);

impl TryFrom<String> for Nickname {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 {
            Ok(Self(value))
        } else {
            Err("Nickname should contain less than 32 characters!")
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct Tech(String);

impl TryFrom<String> for Tech {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= 32 {
            Ok(Self(value))
        } else {
            Err("Each stack should contain less than 32 characters!")
        }
    }
}

impl From<Tech> for String {
    fn from(value: Tech) -> Self {
        value.0
    }
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    pub name: Name,
    pub nickname: Nickname,
    #[serde(with = "date_format")]
    pub birthdate: Date,
    pub stack: Option<Vec<Tech>>,
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
    let id = Uuid::now_v7();

    let person: Person = Person {
        id,
        name: new_person.name.0,
        nickname: new_person.nickname.0,
        birthdate: new_person.birthdate,
        stack: new_person
            .stack
            .map(|stack| stack.into_iter().map(String::from).collect()),
    };

    people.write().await.insert(id, person.clone());

    (StatusCode::OK, Json(person))
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    let count = people.read().await.len();
    (StatusCode::NOT_FOUND, Json(count));
}
