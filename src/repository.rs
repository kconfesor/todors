use crate::state::Repository;
use crate::model::{Todo, NewTodo};
use crate::schema::todo::dsl::*;
use crate::messages::{FetchTodos, CreateTodo, GetTodo, UpdateTodoMessage, DeleteTodo};
use actix::Handler;
use diesel::{self, prelude::*};
const CONNECTION_ERROR: &str = "Unable to establish connection";

impl Handler<FetchTodos> for Repository {
    type Result = QueryResult<Vec<Todo>>;

    fn handle(&mut self, _msg: FetchTodos, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()
            .expect(CONNECTION_ERROR);

        todo.get_results::<Todo>(&mut conn)
    }
}

impl Handler<GetTodo> for Repository {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, _msg: GetTodo, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()
            .expect(CONNECTION_ERROR);

        todo.filter(id.eq(_msg.id))
            .get_result::<Todo>(&mut conn)
    }
}

impl Handler<CreateTodo> for Repository {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, _msg: CreateTodo, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()
            .expect(CONNECTION_ERROR);

        let new_todo = NewTodo {
            title: _msg.title,
            description: _msg.description,
            user_email: _msg.user_email,
            created_at: chrono::Utc::now().naive_utc(),
            enabled: true,
            completed: false,
        };

        diesel::insert_into(todo)
            .values(new_todo)
            .get_result::<Todo>(&mut conn)
    }
}

impl Handler<UpdateTodoMessage> for Repository {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, _msg: UpdateTodoMessage, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()
            .expect(CONNECTION_ERROR);

        diesel::update(todo.filter(id.eq(_msg.id)))
            .set((
                title.eq(_msg.todo.title),
                description.eq(_msg.todo.description),
                completed.eq(_msg.todo.completed),
                enabled.eq(_msg.todo.enabled),
                due_at.eq(_msg.todo.due_at),
                updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<Todo>(&mut conn)
    }
}

impl Handler<DeleteTodo> for Repository {
    type Result = QueryResult<usize>;

    fn handle(&mut self, _msg: DeleteTodo, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()
            .expect(CONNECTION_ERROR);

        let stored =  todo.filter(id.eq(_msg.id))
            .get_result::<Todo>(&mut conn);

        if stored.is_err() {
            return Err(diesel::result::Error::NotFound);
        }

        diesel::delete(todo.filter(id.eq(_msg.id)))
            .execute(&mut conn)
    }
}