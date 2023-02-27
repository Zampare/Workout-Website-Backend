use dotenv;
extern crate pretty_env_logger;
use sqlx::{query, PgPool, Pool, postgres::PgPoolOptions, Postgres};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
mod services;
use services::{self, log_user_workout};
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
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(AppState{db: pool.clone()}))
            .service(log_user_workout)
            /*.service()
            .service()*/
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

