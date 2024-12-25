use uuid::Uuid;
use crate::model::todo::Todo;
use crate::repository::todo_repository::TodoRepository;

pub struct TodoService {
    repository: TodoRepository,
}

impl TodoService {
    pub fn new(repository: TodoRepository) -> Self {
        Self { repository }
    }

    pub async fn create_todo(&self, todo: Todo) -> Result<Todo, String> {
        if todo.title.len() < 3 {
            return Err("Title must be at least 3 characters long".to_string());
        }

        if let Ok(Some(_)) = self.repository.find_by_title(&todo.title).await {
            return Err("Todo with this title already exists".to_string());
        }

        let new_todo = Todo {
            id: Some(Uuid::new_v4().to_string().replace("-", "")),
            title: todo.title,
        };

        self.repository
            .create(new_todo)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>, String> {
        self.repository
            .find_all()
            .await
            .map_err(|e| e.to_string())
    }
}