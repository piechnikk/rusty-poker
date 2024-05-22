use std::collections::HashMap;
use uuid::Uuid;
use crate::poker::player::{Player, PlayerAction};
use rand::{thread_rng, Rng};
use super::player::PlayerState;

#[derive(Clone)]
pub struct Game {
    players: HashMap<Uuid, Player>,
    players_by_seats: Vec<Option<Uuid>>,
    pub small_blind: u64,
    pub big_blind: u64, // typically 2 * small_blind, but not always
    pub initial_balance: u64,
    deck: [Card; 52],
    community_cards: [Card; 5],
    dealer_seat: usize,
    after_big_blind_seat: usize,
    active_player: usize,
    pub max_players: usize,
    game_phase: GamePhase
}

fn next_player(players_by_seats: &Vec<Option<Uuid>>, active_player: usize, max_players: usize) -> usize {
    for seat in (active_player + 1)..max_players {
        match players_by_seats[seat] {
            Some(_) => return seat,
            _ => ()
        }
    }

    for seat in 0..max_players {
        match players_by_seats[seat] {
            Some(_) => return seat,
            _ => ()
        }
    }

    0
}

impl Game {
    pub fn new_game(max_players: usize, small_blind: u64, big_blind: u64, initial_balance: u64) -> Game {
        let players: HashMap<Uuid, Player> = HashMap::with_capacity(max_players);
        let players_by_seats = vec![None; max_players];
        let deck: [Card; 52] = [
            Card { color: Color::Spades, rank: Rank::Two },
            Card { color: Color::Spades, rank: Rank::Three },
            Card { color: Color::Spades, rank: Rank::Four },
            Card { color: Color::Spades, rank: Rank::Five },
            Card { color: Color::Spades, rank: Rank::Six },
            Card { color: Color::Spades, rank: Rank::Seven },
            Card { color: Color::Spades, rank: Rank::Eight },
            Card { color: Color::Spades, rank: Rank::Nine },
            Card { color: Color::Spades, rank: Rank::Ten },
            Card { color: Color::Spades, rank: Rank::Jack },
            Card { color: Color::Spades, rank: Rank::Queen },
            Card { color: Color::Spades, rank: Rank::King },
            Card { color: Color::Spades, rank: Rank::Ace },
            Card { color: Color::Hearts, rank: Rank::Two },
            Card { color: Color::Hearts, rank: Rank::Three },
            Card { color: Color::Hearts, rank: Rank::Four },
            Card { color: Color::Hearts, rank: Rank::Five },
            Card { color: Color::Hearts, rank: Rank::Six },
            Card { color: Color::Hearts, rank: Rank::Seven },
            Card { color: Color::Hearts, rank: Rank::Eight },
            Card { color: Color::Hearts, rank: Rank::Nine },
            Card { color: Color::Hearts, rank: Rank::Ten },
            Card { color: Color::Hearts, rank: Rank::Jack },
            Card { color: Color::Hearts, rank: Rank::Queen },
            Card { color: Color::Hearts, rank: Rank::King },
            Card { color: Color::Hearts, rank: Rank::Ace },
            Card { color: Color::Diamonds, rank: Rank::Two },
            Card { color: Color::Diamonds, rank: Rank::Three },
            Card { color: Color::Diamonds, rank: Rank::Four },
            Card { color: Color::Diamonds, rank: Rank::Five },
            Card { color: Color::Diamonds, rank: Rank::Six },
            Card { color: Color::Diamonds, rank: Rank::Seven },
            Card { color: Color::Diamonds, rank: Rank::Eight },
            Card { color: Color::Diamonds, rank: Rank::Nine },
            Card { color: Color::Diamonds, rank: Rank::Ten },
            Card { color: Color::Diamonds, rank: Rank::Jack },
            Card { color: Color::Diamonds, rank: Rank::Queen },
            Card { color: Color::Diamonds, rank: Rank::King },
            Card { color: Color::Diamonds, rank: Rank::Ace },
            Card { color: Color::Clubs, rank: Rank::Two },
            Card { color: Color::Clubs, rank: Rank::Three },
            Card { color: Color::Clubs, rank: Rank::Four },
            Card { color: Color::Clubs, rank: Rank::Five },
            Card { color: Color::Clubs, rank: Rank::Six },
            Card { color: Color::Clubs, rank: Rank::Seven },
            Card { color: Color::Clubs, rank: Rank::Eight },
            Card { color: Color::Clubs, rank: Rank::Nine },
            Card { color: Color::Clubs, rank: Rank::Ten },
            Card { color: Color::Clubs, rank: Rank::Jack },
            Card { color: Color::Clubs, rank: Rank::Queen },
            Card { color: Color::Clubs, rank: Rank::King },
            Card { color: Color::Clubs, rank: Rank::Ace }
        ];
        let community_cards: [Card; 5] = [ // dummy values replaced later
            Card { color: Color::Spades, rank: Rank::Two },
            Card { color: Color::Spades, rank: Rank::Two },
            Card { color: Color::Spades, rank: Rank::Two },
            Card { color: Color::Spades, rank: Rank::Two },
            Card { color: Color::Spades, rank: Rank::Two }
        ];
        Game { 
            players, 
            small_blind, 
            big_blind, 
            initial_balance, 
            deck, 
            community_cards, 
            players_by_seats, 
            dealer_seat: 0,
            after_big_blind_seat: 0,
            active_player: 0,
            max_players,
            game_phase: GamePhase::PreFlop
        }
    }

    pub fn join_game(
        &mut self,
        seat_index: u8,
        appearance_type: u8
    ) -> Result<Uuid, &str> {
        if self.players_by_seats[seat_index as usize] != None {
            return Err("seat already taken");
        }
        let player_id = Uuid::new_v4();
        let player = Player::new_player(seat_index, self.initial_balance, appearance_type);
        self.players.insert(player_id, player);
        Ok(player_id)
    }

    // you always have to check if this method gives error and if not, call game.set_next_active_player();
    pub fn player_action(&mut self, player_id: Uuid, action: PlayerAction, amount: u64) -> Result<u64, &str> {
        let player = match self.players.get_mut(&player_id) {
            None => return Err("player not found"),
            Some(player) => {
                if player.seat_index != self.active_player as u8 {
                    return Err("it is not your turn");
                }
                player
            }
        };
        
        let result = player.perform_action(action, amount)?;

        Ok(result)
        // let player = self.players.get_mut(&player_id);
        // match player {
        //     None => Err("player not found"),
        //     Some(player) => {
        //         if player.seat_index != self.active_player as u8 {
        //             return Err("it is not your turn")
        //         }
        //         self.active_player = self.next_player();
        //         player.perform_action(action, amount)
        //     }
        // }
    }

    pub fn start_game(&mut self) -> Result<u64, &str> {
        if self.players_count() < 3 {
            return Err("too few players");
        }
        let mut first_taken_seat: usize = 0;
        for (idx, player_id) in self.players_by_seats.iter().enumerate() {
            match player_id {
                Some(_) => first_taken_seat = idx,
                _ => ()
            }
        }
        self.dealer_seat = first_taken_seat;
        self.deal_cards();

        self.set_next_active_player();

        let message = self.player_action(
            self.players_by_seats[self.active_player].expect("active player is None"), 
            PlayerAction::Bet, 
            self.small_blind
        );
        match message {
            Err(_) => {
                    let _ = self.player_action(
                    self.players_by_seats[self.active_player].expect("active player is None"), 
                    PlayerAction::AllIn, 
                    0
                );
            },
            Ok(_) => ()
        }

        self.set_next_active_player();

        let message = self.player_action(
            self.players_by_seats[self.active_player].expect("active player is None"), 
            PlayerAction::Bet, 
            self.big_blind
        );
        match message {
            Err(_) => {
                    let _ = self.player_action(
                    self.players_by_seats[self.active_player].expect("active player is None"), 
                    PlayerAction::AllIn, 
                    0
                );
            },
            Ok(_) => ()
        }

        self.set_next_active_player();

        Ok(0)
    }

    pub fn set_next_active_player(&mut self) {
        let mut next_player_seat = next_player(&self.players_by_seats, self.active_player, self.max_players);
        let mut next_player_active: bool = self.players.get(&self.players_by_seats[next_player_seat].unwrap()).unwrap().state == PlayerState::Active;
        let mut i = 0;
        while !next_player_active {
            next_player_seat = next_player(
                &self.players_by_seats, 
                (self.active_player + i) % self.max_players, 
                self.max_players
            );
            next_player_active = self.players.get(&self.players_by_seats[next_player_seat].unwrap()).unwrap().state == PlayerState::Active;
            i = i + 1;
            if i > 10 {
                println!("cant find active player!");
                return;
            }
        }
        self.active_player = next_player_seat;
    }

    fn deal_cards(&mut self) {
        self.shuffle();
        let mut next_card = 0;
        for player in self.players.values_mut() {
            player.take_card(0, &self.deck[next_card]);
            player.take_card(0, &self.deck[next_card + 1]);
            next_card += 2;
        }
        for card_offset in 0..4 {
            self.community_cards[next_card + card_offset] = self.deck[next_card + card_offset].clone();
        }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        // Knuth shuffle
        for n in 0..52 {
            let i = rng.gen_range(0..52 - n);
            self.deck.swap(i, 51 - n);
        }
    }

    pub fn players_count(&self) -> u8 {
        let mut count = 0;
        for player_id in &self.players_by_seats {
            match player_id {
                Some(_) => count += 1,
                _ => ()
            }
        }
        count
    }

    fn max_bet(&self) -> u64 {
        let mut max_bet = 0;
        for player in self.players.values() {
            if player.current_bet > max_bet {
                max_bet = player.current_bet;
            }
        }
        max_bet
    }

    fn all_bets_even(&self) -> bool {
        let max_bet = self.max_bet();
        for player in self.players.values() {
            if player.current_bet != max_bet {
                return false;
            }
        }
        true
    }
}

#[derive(Clone)]
pub enum Color {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

#[derive(Clone)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Clone)]
enum GamePhase {
    PreFlop, // every player has 2 cards, 0 community cards
    Flop, // first 3 community cards
    Turn, // 4th community card
    River // 5th community card on the table
}

#[derive(Clone)]
pub struct Card {
    rank: Rank,
    color: Color
}

impl Card {
    pub fn new(color: Color, rank: Rank) -> Card {
        Card{color, rank}
    }
}
