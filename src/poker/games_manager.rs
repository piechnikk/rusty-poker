use std::collections::HashMap;

use crate::poker::game::{Game};
use uuid::Uuid;
// use crate::poker::player;

use std::sync::{RwLock, Arc};

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

    pub fn get_all_games_data(&self) -> Vec<GameData> {
        let mut all_games_data: Vec<GameData> = Vec::new();

        for game in self.games.values() {
            let game_data = GameData{ 
                seats_count: game.max_players, 
                seats_occupied: game.players_count(),
                small_blind: game.small_blind,
                big_blind: game.big_blind,
                initial_balance: game.initial_balance
            };
            all_games_data.push(game_data);
        }

        all_games_data
    }
}

pub struct GameData {
    pub seats_count: usize,
    pub seats_occupied: u8,
    pub small_blind: u64,
    pub big_blind: u64,
    pub initial_balance: u64
}

// Tworzymy alias GamesManagerArc, który jest Arc opakowującym GamesManager
pub type GamesManagerArc = Arc<RwLock<GamesManager>>;