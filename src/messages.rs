use crate::model::{Todo};
use actix::Message;
use diesel::QueryResult;
use serde::Deserialize;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Todo>>")]
pub struct FetchTodos;

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct GetTodo {
    pub id: i32,
}

#[derive(Message, Deserialize)]
#[rtype(result = "QueryResult<Todo>")]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
    pub user_email: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub enabled: bool,
    pub due_at: Option<chrono::NaiveDateTime>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct UpdateTodoMessage {
    pub id: i32,
    pub todo: UpdateTodo,
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeleteTodo {
    pub id: i32,
}