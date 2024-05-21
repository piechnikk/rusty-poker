mod poker {
    pub mod game;
    pub mod games_manager;
    pub mod player;
}
use std::sync::{Arc, RwLock};
use poker::games_manager::GamesManager;
use poker::games_manager::GamesManagerArc;

use actix_cors::Cors;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use uuid::Uuid;

// handlers structures
#[derive(Deserialize, Debug)]
enum Action {
    Raise,
    Call,
    Check,
    Fold,
}

#[derive(Deserialize)]
struct CreateGame {
    seats_count: usize,
    small_blind: u64,
    big_blind: u64,
    initial_balance: u64,
    bet_time: u64,
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

fn check_joined(session: &Session) -> Result<(), HttpResponse> {
    if let Some(joined) = session.get::<bool>("joined").unwrap() {
        if joined {
            Ok(())
        } else {
            Err(HttpResponse::Unauthorized().body("Unauthorized"))
        }
    } else {
        Err(HttpResponse::Unauthorized().body("Unauthorized"))
    }
}

// handlers
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/create_game")]
async fn create_game(data: web::Data<GamesManagerArc>, body: web::Json<CreateGame>) -> impl Responder {
    let mut games_manager = data.write().unwrap();
    let new_game_id = games_manager.new_game(
        body.seats_count,
        body.small_blind,
        body.big_blind,
        body.initial_balance,
    );
    
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
        "games": [{
            "game_id": Uuid::new_v4(),
            "seats_count": 8,
            "seats_occupied": 2,
            "small_blind": 10,
            "big_blind": 20,
            "initial_balance": 100,
            "bet_time": 30
        }]
    });

    HttpResponse::Ok().json(response)
}

#[post("/join_game")]
async fn join_game(session: Session, _body: web::Json<JoinGame>) -> impl Responder {
    session.insert("joined", true).unwrap();
    session.insert("player_id", Uuid::new_v4()).unwrap();

    let response = serde_json::json!({
        "message": "success"
    });

    HttpResponse::Ok().json(response)
}

#[post("/set_ready")]
async fn set_ready(session: Session, _body: web::Json<SetReady>) -> impl Responder {
    if let Err(err) = check_joined(&session) {
        return err;
    }

    let mut response = serde_json::json!({
        "message": "success"
    });

    if let Some(user_id) = session.get::<bool>("user_id").unwrap() {
        response = serde_json::json!({
            "message": "success",
            "user_id": user_id
        });
    }

    return HttpResponse::Ok().json(response);
}

#[get("/game_state")]
async fn game_state(session: Session, query: web::Query<GameId>) -> impl Responder {
    if let Err(_err) = check_joined(&session) {
        let response = serde_json::json!({
            "message": "success",
            "game_state": "unauthorized"
        });

        return HttpResponse::Ok().json(response);
    }

    let response = serde_json::json!({
        "message": "success",
        "game_state": query.game_id
    });

    HttpResponse::Ok().json(response)
}

#[get("/listen_changes")]
async fn listen_changes(session: Session, query: web::Query<GameId>) -> impl Responder {
    // long polling
    if let Err(err) = check_joined(&session) {
        return err;
    }

    let response = serde_json::json!({
        "message": "updated",
        "game_state": query.game_id
    });

    HttpResponse::Ok().json(response)
}

#[post("/perform_action")]
async fn perform_action(session: Session, _body: web::Json<PerformAction>) -> impl Responder {
    if let Err(err) = check_joined(&session) {
        return err;
    }

    let response = serde_json::json!({
        "message": "success"
    });

    HttpResponse::Ok().json(response)
}

#[post("/quit_game")]
async fn quit_game(session: Session, _body: web::Json<GameId>) -> impl Responder {
    if let Err(err) = check_joined(&session) {
        return err;
    }

    session.remove("joined");

    let response = serde_json::json!({
        "message": "success"
    });

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();

    let mut games_manager = Arc::new(RwLock::new(GamesManager::new_manager()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(games_manager.clone()))
            .wrap(SessionMiddleware::new(
                actix_session::storage::CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(Cors::default().allow_any_origin())
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
