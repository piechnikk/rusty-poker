use std::collections::HashMap;
use uuid::Uuid;
use crate::poker::player::{Player};

struct Game {
    players: HashMap<Uuid, Player>,
    small_blind: u64,
    big_blind: u64, // typically 2 * small_blind, but not always
    initial_balance: u64,
}

impl Game {
    pub fn new_game(max_players: usize, small_blind: u64, big_blind: u64, initial_balance: u64) -> Game {
        let players: HashMap<Uuid, Player> = HashMap::with_capacity(max_players);
        Game { players, small_blind, big_blind, initial_balance }
    }

    pub fn join_game(
        &mut self,
        seat_index: u8,
        appearance_type: u8
    ) -> Result<Uuid, &str> {
        for player in self.players.values() {
            if player.seat_index == seat_index {
                return Err("seat already taken");
            }
        }
        let player_id = Uuid::new_v4();
        let player = Player::new_player(seat_index, self.initial_balance, appearance_type);
        self.players.insert(player_id, player);
        Ok(player_id)
    }
}
