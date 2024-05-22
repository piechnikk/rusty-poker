use std::collections::HashMap;

use serde::Serialize;
use crate::poker::game::{Game, Card, Color, Rank};
use crate::poker::player::{Player, PlayerData, PlayerState};
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
                initial_balance: game.initial_balance
            };
            all_games_data.push(game_data);
        }

        all_games_data
    }

    pub fn get_game_state(&self, game_id: Uuid, player_id: Uuid) -> Result<GameState, &str> {
        // let game = self.games.get(&game_id);

        // match

        Ok(GameState{
            community_cards: vec![
                Card::new(Color::Spades, Rank::Two),
                Card::new(Color::Spades, Rank::Two),
                Card::new(Color::Spades, Rank::Two),
            ],
            personal_cards: vec![
                Card::new(Color::Spades, Rank::Two),
                Card::new(Color::Spades, Rank::Two),
            ],
            bets_placed: vec![None, None, Some(30), Some(60), None, Some(60), None],
            pot: 150,
            small_blind: 10,
            big_blind: 20,
            dealer: 2,
            players: vec![
                None,
                None,
                Some(PlayerData{seat_index: 2, balance: 20, state: PlayerState::Active, bet_amount: 30, nickname: "ela".to_string()}),
                Some(PlayerData{seat_index: 2, balance: 20, state: PlayerState::Active, bet_amount: 30, nickname: "ela".to_string()}),
                None,
                Some(PlayerData{seat_index: 2, balance: 20, state: PlayerState::Active, bet_amount: 30, nickname: "ela".to_string()}),
                None
            ]
        })
    }
}

#[derive(Serialize)]
pub struct GameData {
    pub game_id: Uuid,
    pub seats_count: usize,
    pub seats_occupied: u8,
    pub small_blind: u64,
    pub big_blind: u64,
    pub initial_balance: u64
}

#[derive(Serialize)]
pub struct GameState {
    community_cards: Vec<Card>,
    personal_cards: Vec<Card>,
    bets_placed: Vec<Option<u64>>, // indexed by seats
    pot: u64,
    players: Vec<Option<PlayerData>>,
    small_blind: u64,
    big_blind: u64,
    dealer: usize
}

pub type GamesManagerArc = Arc<RwLock<GamesManager>>;
