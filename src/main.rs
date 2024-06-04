mod poker {
    pub mod game;
    pub mod games_manager;
    pub mod player;
}
use poker::game::Card;
use poker::game::Game;
use poker::games_manager::GamesManager;
use poker::games_manager::GamesManagerArc;
use poker::player::Player;
use std::sync::{Arc, RwLock};

use actix_cors::Cors;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use poker::player::PlayerAction;
use serde::Deserialize;
use serde::Serialize;
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
    chosen_seat: u8,
    appearance_type: u8,
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
    bet: Option<u64>,
    action: PlayerAction,
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
async fn create_game(
    data: web::Data<GamesManagerArc>,
    body: web::Json<CreateGame>,
) -> impl Responder {
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
async fn games(data: web::Data<GamesManagerArc>) -> impl Responder {
    let games_manager = data.read().unwrap();
    let games = games_manager.get_all_games_data();
    let response = serde_json::json!({
        "message": "success",
        "games": games
    });

    HttpResponse::Ok().json(response)
}

#[post("/join_game")]
async fn join_game(
    data: web::Data<GamesManagerArc>,
    session: Session,
    body: web::Json<JoinGame>,
) -> impl Responder {
    if let Ok(_) = check_joined(&session) {
        return HttpResponse::Forbidden().json(serde_json::json!({"message": "error"}));
    }

    let mut games_manager = data.write().unwrap();

    if let Ok(game) = games_manager.get_game_mut(body.game_id) {
        match game.join_game(body.chosen_seat, &body.player_name, body.appearance_type) {
            Err(err) => {
                return HttpResponse::Forbidden()
                    .json(serde_json::json!({"message": "error", "content": err}))
            }
            Ok(user_id) => {
                session.insert("joined", true).unwrap();
                session.insert("player_id", user_id).unwrap();
                return HttpResponse::Ok().json(serde_json::json!({"message": "success"}));
            }
        }
    } else {
        HttpResponse::Forbidden().json(serde_json::json!({"message": "error"}))
    }
}

#[post("/set_ready")]
async fn set_ready(
    data: web::Data<GamesManagerArc>,
    session: Session,
    body: web::Json<SetReady>,
) -> impl Responder {
    if let Err(err) = check_joined(&session) {
        return err;
    }

    let mut response = serde_json::json!({
        "message": "success"
    });

    let mut games_manager = data.write().unwrap();
    if let Ok(game) = games_manager.get_game_mut(body.game_id) {
        println!("ddd");
        let ok = game.set_ready(
            session.get::<Uuid>("player_id").unwrap().unwrap(),
            body.new_ready_state,
        );
        println!("{:?}", ok);
        response = serde_json::json!({
            "message": "success",
        });
    }

    return HttpResponse::Ok().json(response);
}

#[get("/game_state")]
async fn game_state(
    data: web::Data<GamesManagerArc>,
    session: Session,
    query: web::Query<GameId>,
) -> impl Responder {
    let games_manager = data.write().unwrap();

    if let Err(_err) = check_joined(&session) {
        let game_state = games_manager.get_game_state(query.game_id, Uuid::new_v4());

        match game_state {
            Err(err) => {
                return HttpResponse::Forbidden()
                    .json(serde_json::json!({"message": "error", "content": err }))
            }
            Ok(game) => {
                return HttpResponse::Ok()
                    .json(serde_json::json!({"message": "success", "game_state": game}))
            }
        }
    }
    let game_state = games_manager.get_game_state(
        query.game_id,
        session.get::<Uuid>("player_id").unwrap().unwrap(),
    );

    match game_state {
        Err(err) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"message": "error", "content": err }))
        }
        Ok(game) => {
            return HttpResponse::Ok()
                .json(serde_json::json!({"message": "success", "game_state": game}))
        }
    }
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
async fn perform_action(
    data: web::Data<GamesManagerArc>,
    session: Session,
    body: web::Json<PerformAction>,
) -> impl Responder {
    if let Err(err) = check_joined(&session) {
        return err;
    }
    let mut games_manager = data.write().unwrap();
    if let Ok(game) = games_manager.get_game_mut(body.game_id) {
        let player_index = game
            .players
            .get(&session.get::<Uuid>("player_id").unwrap().unwrap())
            .unwrap();
        game.player_action(*player_index, body.action, body.bet.unwrap());
    } else {
        return HttpResponse::Forbidden().json(serde_json::json!({"message": "error"}));
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
            .wrap(Cors::permissive())
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
