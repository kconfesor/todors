use actix::SyncArbiter;
use actix_web::{middleware::Logger,App, web, HttpServer};
use self::services::*;
use diesel::{
    r2d2::{Pool, ConnectionManager},
    pg::PgConnection,
};
use dotenvy::dotenv;
use std::env;
use state::{get_pool, AppState, Repository};
use diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

//modules
mod model;
mod services;
mod schema;
mod state;
mod messages;
mod repository;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&database_url);
    run_migrations(&pool);
    let db_addr = SyncArbiter::start(5, move || Repository(pool.clone()));
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("verbose"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db_addr.clone() }))
            .wrap(Logger::default())
            .service(get_todos)
            .service(get_todo)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
        })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

fn run_migrations(pool: &Pool<ConnectionManager<PgConnection>>) {
    let mut conn = pool.get().expect("Failed to get a connection");
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("Migrations applied successfully"),
        Err(e) => panic!("Failed to apply migrations: {:?}", e),
    }
}