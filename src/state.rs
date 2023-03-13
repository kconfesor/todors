use actix::{Actor, Addr, SyncContext};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool}
};

pub struct AppState {
    pub db: Addr<Repository>
}

pub struct Repository(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Repository {
    type Context = SyncContext<Self>;
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Error building a connection pool")
}