use dotenv;
extern crate pretty_env_logger;
use sqlx::{query, PgPool, Pool, postgres::PgPoolOptions, Postgres};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
mod services;
use services::{log_user_workout, pull_user_lifts};
pub struct AppState {
    db: Pool<Postgres>
}
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.expect("Error building a connection pool");
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS lifts (id SERIAL PRIMARY KEY NOT NULL, lift VARCHAR(250) NOT NULL, reps INTEGER, weight INTEGER, rpe INTEGER, time TIMESTAMPTZ);").execute(&pool).await.unwrap();
    println!("Create user table result: {:?}", result);
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(AppState{db: pool.clone()}))
            .service(log_user_workout)
            .service(pull_user_lifts)
            /*.service()
            .service()*/
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

