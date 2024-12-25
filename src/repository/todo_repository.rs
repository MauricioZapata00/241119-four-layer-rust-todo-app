use mongodb::{Collection, error::Error};
use mongodb::bson::doc;
use futures::TryStreamExt;
use crate::model::todo::Todo;

pub struct TodoRepository {
    collection: Collection<Todo>,
}

impl TodoRepository {
    pub fn new(collection: Collection<Todo>) -> Self {
        Self { collection }
    }

    pub async fn create(&self, todo: Todo) -> Result<Todo, Error> {
        self.collection.insert_one(todo.clone()).await?;
        Ok(todo)
    }

    pub async fn find_by_title(&self, title: &str) -> Result<Option<Todo>, Error> {
        self.collection.find_one(doc! { "title": title }).await
    }

    pub async fn find_all(&self) -> Result<Vec<Todo>, Error> {
        let cursor = self.collection.find(doc! {}).await?;
        cursor.try_collect().await
    }
}