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




#[post("/workout/lifts")]
pub async fn log_user_workout(state: Data<AppState>, body: Json<LogLift>) -> impl Responder{
    
    match sqlx::query_as::<_, Lift>( "INSERT INTO lifts (lift, weight, reps, rpe, time) VALUES ($1, $2, $3, $4, $5) RETURNING id, lift, weight, reps, rpe, time")
        .bind(body.lift.to_string())
        .bind(body.weight)
        .bind(body.reps)
        .bind(body.rpe)
        .bind(body.time)
        .fetch_one(&state.db)
        .await{
            Ok(lift) => HttpResponse::Ok().json(lift),
            Err(e) => {println!("{:?}", e);HttpResponse::InternalServerError().json("Failed to create user lift")},
        }
}

#[get("/workout/lifts")]
pub async fn pull_user_lifts(state: Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, Lift>(
        "SELECT * FROM lifts"
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(lift) => HttpResponse::Ok().json(lift),
        Err(e) => {println!("{:?}", e);HttpResponse::NotFound().json("No lifts found")},
    }

}//{"lift":"BENCH","weight":225,"reps":3,"rpe":9,"time":"2023-04-05T18:44:00Z"}