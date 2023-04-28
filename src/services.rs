use actix_web::{get, post, web::{Data, Json, Path}, Responder, HttpResponse, put, delete};
use crate::{
    AppState, Repository,
};
use actix::Addr;
use crate::messages::{CreateTodo, DeleteTodo, FetchTodos, GetTodo, UpdateTodo, UpdateTodoMessage};
use email_address::*;

#[get("/todo/all")]
pub async fn get_todos(state: Data<AppState>) -> impl Responder {
    let db: Addr<Repository> = state.as_ref().db.clone();

    match db.send(FetchTodos).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("Todos list is empty"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve todos"),
    }
}

#[get("/todo/{id}")]
pub async fn get_todo(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let db: Addr<Repository> = state.as_ref().db.clone();
    let id: i32 = path.into_inner();

    match db.send(GetTodo { id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("Todo with id {} not found", id)),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve todo"),
    }
}

#[post("/todo")]
pub async fn create_todo(state: Data<AppState>, body: Json<CreateTodo>) -> impl Responder {
    let db: Addr<Repository> = state.as_ref().db.clone();

    match body
    {
        body if body.title.is_empty() => HttpResponse::BadRequest().json("Title field is empty"),
        body if body.user_email.is_empty() => HttpResponse::BadRequest().json("User email field is empty"),
        body if !EmailAddress::is_valid(&body.user_email) => HttpResponse::BadRequest().json("User email is not valid"),
        _ => match db.send(CreateTodo {
            title: body.title.to_string(),
            description: body.description.to_string(),
            user_email: body.user_email.to_string(),
        }).await {
            Ok(Ok(info)) => HttpResponse::Ok().json(info),
            Ok(Err(err)) => HttpResponse::BadRequest().json(format!("Todo with id {} not found", err.to_string())),
            _ => HttpResponse::InternalServerError().json("Unable to create todo"),
        }
    }
}

#[put("/todo/{id}")]
pub async fn update_todo(state: Data<AppState>, path: Path<i32>, body: Json<UpdateTodo>) -> impl Responder {
    let db: Addr<Repository> = state.as_ref().db.clone();
    let id: i32 = path.into_inner();

    match body
    {
        body if body.title.is_empty() => HttpResponse::BadRequest().json("Title field is empty"),
        _ => match db.send(UpdateTodoMessage {
            id,
            todo: UpdateTodo {
                title: body.title.to_string(),
                description: body.description.to_string(),
                completed: body.completed,
                enabled: body.enabled,
                due_at: body.due_at,
            },
        }).await {
            Ok(Ok(info)) => HttpResponse::Ok().json(info),
            Ok(Err(_)) => HttpResponse::NotFound().json(format!("Todo with id {} not found", id)),
            _ => HttpResponse::InternalServerError().json("Unable to update todo"),
        }
    }
}

#[delete("/todo/{id}")]
pub async fn delete_todo(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let db: Addr<Repository> = state.as_ref().db.clone();
    let id: i32 = path.into_inner();

    match db.send(DeleteTodo { id }).await {
        Ok(Ok(_)) => HttpResponse::NoContent().json(()),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("Todo with id {} not found", id)),
        _ => HttpResponse::InternalServerError().json("Unable to delete todo"),
    }
}