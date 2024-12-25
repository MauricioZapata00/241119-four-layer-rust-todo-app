#[macro_use]
extern crate rocket;

mod model;
mod repository;
mod service;
mod controller;

use mongodb::Client;
use rocket::{State, http::Status, response::status, serde::json::Json};
use crate::controller::todo_controller::TodoController;
use crate::model::todo::Todo;
use crate::repository::todo_repository::TodoRepository;
use crate::service::todo_service::TodoService;

#[post("/todos", data = "<todo>")]
async fn create_todo(controller: &State<TodoController>, todo: Json<Todo>) -> Result<status::Created<Json<Todo>>, Status> {
    controller.create_todo(todo).await
}

#[get("/todos")]
async fn get_todos(controller: &State<TodoController>) -> Result<Json<Vec<Todo>>, Status> {
    controller.get_todos().await
}

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let client = Client::with_uri_str("mongodb://LocalDeveloper:f8104adbecc0ec@localhost:27017/")
        .await
        .expect("Failed to connect to MongoDB");

    let db = client.database("db_todos");
    let collection = db.collection("todos");

    let repository = TodoRepository::new(collection);
    let service = TodoService::new(repository);
    let controller = TodoController::new(service);

    rocket::build()
        .manage(controller)
        .mount("/", routes![create_todo, get_todos])
}
