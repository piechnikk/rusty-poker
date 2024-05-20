mod poker;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
enum Action {
    raise,
    call,
    check,
    fold,
}

#[derive(Deserialize)]
struct CreateGame {
    seats_count: i32,
    small_blind: i32,
    big_blind: i32,
    initial_balance: i32,
    bet_time: i32,
}

#[derive(Deserialize)]
struct JoinGame {
    game_id: Uuid,
    player_name: String,
    chosen_seat: i32,
}

#[derive(Deserialize)]
struct SetReady {
    game_id: Uuid,
    new_ready_state: bool,
}

#[derive(Deserialize)]
struct GameId {
    game_id: Uuid,
}

#[derive(Deserialize)]
struct PerformAction {
    game_id: Uuid,
    bet: Option<i32>,
    action: Action,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/create_game")]
async fn create_game(body: web::Json<CreateGame>) -> impl Responder {
    let new_game_id = Uuid::new_v4();

    let response = serde_json::json!({
        "message": "success",
        "game_id": new_game_id
    });

    HttpResponse::Ok().json(response)
}

#[get("/games")]
async fn games() -> impl Responder {
    let response = serde_json::json!({
        "message": "success",
        "games": [0]
    });

    HttpResponse::Ok().json(response)
}

#[post("/join_game")]
async fn join_game(body: web::Json<JoinGame>) -> impl Responder {
    let response = serde_json::json!({
        "message": "success"
    });

    HttpResponse::Ok().json(response)
}

#[post("/set_ready")]
async fn set_ready(body: web::Json<SetReady>) -> impl Responder {
    let response = serde_json::json!({
        "message": "success"
    });

    HttpResponse::Ok().json(response)
}

#[get("/game_state")]
async fn game_state(query: web::Query<GameId>) -> impl Responder {
    let response = serde_json::json!({
        "message": "success",
        "game_state": query.game_id
    });

    HttpResponse::Ok().json(response)
}

#[get("/listen_changes")]
async fn listen_changes(query: web::Query<GameId>) -> impl Responder {
    let response = serde_json::json!({
        "message": "updated",
        "game_state": query.game_id
    });

    HttpResponse::Ok().json(response)
}

#[post("/perform_action")]
async fn perform_action(body: web::Json<PerformAction>) -> impl Responder {
    let response = serde_json::json!({
        "message": "success"
    });

    HttpResponse::Ok().json(response)
}

#[post("/quit_game")]
async fn quit_game(body: web::Json<GameId>) -> impl Responder {
    let response = serde_json::json!({
        "message": "success"
    });

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(create_game)
            .service(games)
            .service(join_game)
            .service(set_ready)
            .service(game_state)
            .service(listen_changes)
            .service(perform_action)
            .service(quit_game)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
