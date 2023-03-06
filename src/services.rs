use actix_web::{get, post, web::{Data, Json, Path}, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;
use chrono;

#[derive(Serialize, FromRow)]
struct User{
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, FromRow)]
struct Lift{
    id:i32,
    lift: String,
    weight:i32,
    reps:i32,
    rpe:i32,
    time:chrono::DateTime<chrono::Utc>
}
    
    

#[derive(Deserialize)]
pub struct LogLift{
    pub lift: String,
    pub weight:i32,
    pub reps:i32,
    pub rpe:i32,
    pub time:chrono::DateTime<chrono::Utc>
}




#[post("/workout/{id}/lifts")]
pub async fn log_user_workout(state: Data<AppState>, path: Path<i32>, body: Json<LogLift>) -> impl Responder{
    let id:i32 = path.into_inner();
    
    match sqlx::query_as::<_, Lift>( "INSERT INTO lifts (id, lift, weight, reps, rpe, time) VALUES ($1, $2, $3, $4, $5) RETURNING id, lift, weight, reps, rpe, time")
        .bind(id)
        .bind(body.lift.to_string())
        .bind(body.weight)
        .bind(body.reps)
        .bind(body.rpe)
        .bind(body.time)
        .fetch_one(&state.db)
        .await{
            Ok(lift) => HttpResponse::Ok().json(lift),
            Err(_) => HttpResponse::InternalServerError().json("Failed to create user lift"),
        }
}

#[get("/workout/{id}/lifts")]
pub async fn pull_user_lifts(state: Data<AppState>, path: Path<i32>) -> impl Responder{
    let id:i32 = path.into_inner();

    match sqlx::query_as::<_, Lift>(
        "SELECT id, lift, weight, reps, rpe, time FROM articles WHERE id = $1"
    )
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Ok(lift) => HttpResponse::Ok().json(articles),
        Err(_) => HttpResponse::NotFound().json("No lifts found"),
    }

}