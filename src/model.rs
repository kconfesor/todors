use super::schema::todo;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Todo {
    pub id: i32,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_at: Option<chrono::NaiveDateTime>,
    pub enabled: bool,
    pub user_email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = todo)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub user_email: String,
    pub created_at: chrono::NaiveDateTime,
    pub enabled: bool,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub title: String,
    pub temp: i32,
    pub humidity: i32,
    pub enabled: bool,
}