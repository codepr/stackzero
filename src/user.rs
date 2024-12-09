use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use actix_web::{get, post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub nickname: String,
}

impl User {
    pub fn new(email: String, nickname: String) -> Self {
        Self { email, nickname }
    }
}

#[get("/users")]
pub async fn list() -> HttpResponse {
    HttpResponse::Created()
        .content_type("application/json")
        .json(User::new(
            String::from("hello@gmail.com"),
            String::from("hello"),
        ))
}

/// Find a User by id
#[get("/users/{id}")]
pub async fn get(_path: Path<(String,)>) -> HttpResponse {
    let user: Option<User> = None;

    match user {
        Some(user) => HttpResponse::Ok()
            .content_type("application/json")
            .json(user),
        None => HttpResponse::NoContent()
            .content_type("application/json")
            .await
            .unwrap(),
    }
}

#[post("/users")]
pub async fn create(new_user: Json<User>) -> HttpResponse {
    HttpResponse::Created()
        .content_type("application/json")
        .json(new_user)
}
