use std::collections::HashMap;

use crate::poker::game::{Game};
use uuid::Uuid;
// use crate::poker::player;

struct GamesManager {
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
}
