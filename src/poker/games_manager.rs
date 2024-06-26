use std::collections::HashMap;

use serde::Serialize;
use crate::poker::game::{Game, Card};
use crate::poker::player::{PlayerData};
use uuid::Uuid;

// use crate::poker::player;

use std::sync::{RwLock, Arc};

use super::game::GamePlayState;

#[derive(Clone)]
pub struct GamesManager {
    games: HashMap<Uuid, Game>
}

impl GamesManager {
    pub fn new_manager() -> GamesManager {
        let games: HashMap<Uuid, Game> = HashMap::with_capacity(100);
        GamesManager{games}
    }

    pub fn new_game(&mut self, max_players: usize, small_blind: u64, big_blind: u64, initial_balance: u64) -> Uuid {
        let game_id = Uuid::new_v4();
        let game = Game::new_game(max_players, small_blind, big_blind, initial_balance);
        self.games.insert(game_id, game);
        game_id
    }

    pub fn get_game(&self, game_id: Uuid) -> Result<&Game, &str> {
        let game = self.games.get(&game_id);
        match game {
            None => Err("game not found"),
            Some(game) => Ok(game)
        }
    }

    pub fn get_game_mut(&mut self, game_id: Uuid) -> Result<&mut Game, &str> {
        let game = self.games.get_mut(&game_id);
        match game {
            None => Err("game not found"),
            Some(game) => Ok(game)
        }
    }

    pub fn get_all_games_data(&self) -> Vec<GameData> {
        let mut all_games_data: Vec<GameData> = Vec::new();

        for (game_id, game) in &self.games {
            let game_data = GameData{
                game_id: *game_id,
                seats_count: game.max_players, 
                seats_occupied: game.players_count(),
                small_blind: game.small_blind,
                big_blind: game.big_blind,
                initial_balance: game.initial_balance,
                game_state: game.game_state
            };
            all_games_data.push(game_data);
        }

        all_games_data
    }

    pub fn get_game_state(&self, game_id: Uuid, player_id: Uuid) -> Result<GameState, &str> {
        let game = self.games.get(&game_id);

        match game {
            None => return Err("game not found"),
            Some(game) => Ok(game.collect_state_data(player_id))
        }
    }
}

#[derive(Serialize)]
pub struct GameData {
    pub game_id: Uuid,
    pub seats_count: usize,
    pub seats_occupied: u8,
    pub small_blind: u64,
    pub big_blind: u64,
    pub initial_balance: u64,
    pub game_state: GamePlayState
}

#[derive(Serialize)]
pub struct GameState {
    pub asker_seat: Option<usize>,
    pub active_seat: usize,
    pub community_cards: [Option<Card>; 5],
    pub personal_cards: [Option<Card>; 2],
    pub bets_placed: Vec<Option<u64>>, // indexed by seats
    pub pot: u64,
    pub players: Vec<Option<PlayerData>>,
    pub small_blind: u64,
    pub big_blind: u64,
    pub game_state: GamePlayState,
    pub dealer_seat: usize,
    pub small_blind_seat: usize,
    pub big_blind_seat: usize
}

pub type GamesManagerArc = Arc<RwLock<GamesManager>>;
