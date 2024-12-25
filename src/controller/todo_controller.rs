use rocket::{State, http::Status, response::status, serde::json::Json};
use crate::model::todo::Todo;
use crate::service::todo_service::TodoService;

pub struct TodoController {
    service: TodoService,
}

impl TodoController {
    pub fn new(service: TodoService) -> Self {
        Self { service }
    }

    pub async fn create_todo(&self, todo: Json<Todo>) -> Result<status::Created<Json<Todo>>, Status> {
        match self.service.create_todo(todo.into_inner()).await {
            Ok(created_todo) => Ok(status::Created::new("/").body(Json(created_todo))),
            Err(_) => Err(Status::BadRequest),
        }
    }

    pub async fn get_todos(&self) -> Result<Json<Vec<Todo>>, Status> {
        match self.service.get_all_todos().await {
            Ok(todos) => Ok(Json(todos)),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}