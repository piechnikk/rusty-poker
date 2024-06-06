use std::collections::HashMap;
use serde::Serialize;
use uuid::Uuid;
use crate::poker::player::{Player, PlayerAction, PlayerData};
use crate::poker::games_manager::{GameState};
use rand::{thread_rng, Rng};
use super::player::PlayerState;
use poker::{Evaluator, Eval, Card as EvaluatorCard, Rank as EvaluatorRank, Suit as EvaluatorColor};

#[derive(Clone)]
pub struct Game {
    pub players: HashMap<Uuid, usize>, // map player_id to his seat index
    pub players_by_seats: Vec<Option<Player>>,
    nicknames: Vec<Option<String>>,
    pub max_players: usize,
    pub small_blind: u64,
    pub big_blind: u64, // typically 2 * small_blind, but not always
    pub initial_balance: u64,
    pub game_state: GamePlayState,
    deck: [Card; 52],
    community_cards: [Option<Card>; 5],
    community_cards_shown: usize,
    dealer_seat: usize,
    active_player: usize,
    game_phase: GamePhase,
    evaluator: Evaluator
}

fn next_player(players_by_seats: &Vec<Option<Player>>, start_from: usize, max_players: usize) -> usize {
    for seat in (start_from + 1)..max_players {
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
        let players: HashMap<Uuid, usize> = HashMap::with_capacity(max_players);
        let players_by_seats = vec![None; max_players];
        let nicknames = vec![None; max_players];
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
        let community_cards: [Option<Card>; 5] = [None, None, None, None, None];
        Game { 
            players, 
            small_blind, 
            big_blind, 
            initial_balance,
            game_state: GamePlayState::NotStarted,
            deck, 
            community_cards,
            community_cards_shown: 0,
            players_by_seats, 
            dealer_seat: 69,
            active_player: 69,
            max_players,
            game_phase: GamePhase::PreFlop,
            evaluator: Evaluator::new(),
            nicknames
        }
    }

    pub fn join_game(
        &mut self,
        seat_index: u8,
        nickname: &str,
        appearance_type: u8
    ) -> Result<Uuid, &str> {
        match self.players_by_seats[seat_index as usize] {
            Some(_) => return Err("seat already taken"),
            _ => ()
        }
        if self.game_state != GamePlayState::NotStarted {
            return Err("game already started or ended");
        }
        let player_id = Uuid::new_v4();
        let player = Player::new_player(seat_index, self.initial_balance, appearance_type);
        self.players.insert(player_id, seat_index as usize);
        self.players_by_seats[seat_index as usize] = Some(player);

        self.nicknames[seat_index as usize] = Some(nickname.to_string());

        if self.players.len() == self.max_players {
            let _ = self.start_game();
        }

        Ok(player_id)
    }

    pub fn set_ready(&mut self, player_index: usize, ready: bool) -> Result<bool, &str> {
        if self.game_state != GamePlayState::NotStarted {
            return Err("game already started or ended");
        }

        let player: &mut Player = self.players_by_seats[player_index].as_mut().unwrap();
        player.set_ready(ready);
        let _ = self.start_game();

        Ok(ready)
    }

    pub fn player_action(&mut self, player_index: usize, action: PlayerAction, amount: u64) -> u8 {        
        if player_index != self.active_player {
            return 0;
        }

        let max_bet = self.max_bet();
        let player: &mut Player = self.players_by_seats[player_index].as_mut().unwrap();
        
        if amount + player.current_bet < max_bet { return 0; }

        let result = match action {
            PlayerAction::Call | PlayerAction::Check => player.perform_action(action, max_bet),
            PlayerAction::Bet => player.perform_action(action, amount),
            PlayerAction::AllIn | PlayerAction::Fold => player.perform_action(action, amount)
        };

        match result {
            Err(_) => return 0,
            Ok(_) => ()
        }
        
        let round_end = self.round_end();
        if !round_end { self.set_next_active_player(); }

        if round_end {
            match self.game_phase {
                GamePhase::PreFlop => {
                    self.game_phase = GamePhase::Flop;
                    self.set_players_active(false);
                    self.community_cards_shown = 3;
                    self.collect_bets();
                },
                GamePhase::Flop => {
                    self.game_phase = GamePhase::Turn;
                    self.set_players_active(false);
                    self.community_cards_shown = 4;
                    self.collect_bets();
                },
                GamePhase::Turn => {
                    self.game_phase = GamePhase::River;
                    self.set_players_active(false);
                    self.community_cards_shown = 5;
                    self.collect_bets();
                },
                GamePhase::River => {
                    self.collect_bets();
                    let winner_seat = self.get_winner_seat();
                    self.distribute_winnings(winner_seat);
                    self.game_phase = GamePhase::PreFlop;
                    self.community_cards_shown = 0;
                    self.start_round(false);
                },
            }
        }

        println!("current round phase is {:?}", self.game_phase);

        1

        // Ok(result)
    }

    pub fn collect_state_data(&self, player_id: Uuid) -> GameState {
        let player_seat = self.players.get(&player_id);
        let mut cards_to_show = self.community_cards;
        for idx in self.community_cards_shown..5 {
            cards_to_show[idx] = None;
        }

        GameState{
            asker_seat: player_seat.copied(),
            active_seat: self.active_player,
            community_cards: cards_to_show,
            personal_cards: match player_seat {
                Some(player_index) if self.game_state == GamePlayState::Started => self.players_by_seats[*player_index].unwrap().cards.map(|card| Some(card)),
                None | Some(_) => [None, None]
            },
            bets_placed: vec![None; self.max_players],
            pot: self.players_by_seats.iter().map(
                |opt_player| match opt_player {
                    Some(player) => player.current_bet + player.total_bet,
                    None => 0
                }
            ).sum(),
            small_blind: self.small_blind,
            big_blind: self.big_blind,
            game_state: self.game_state,
            dealer: self.dealer_seat,
            players: self.players_by_seats.iter().map(
                |opt_player| match opt_player {
                    Some(player) => Some(
                        PlayerData{
                            seat_index: player.seat_index, 
                            balance: player.balance, 
                            state: player.state, 
                            bet_amount: player.current_bet, 
                            nickname: self.nicknames[player.seat_index as usize].as_ref().unwrap().to_string()
                        }
                    ),
                    None => None
                }
            ).collect()
        }
    }

    pub fn start_game(&mut self) -> Result<u64, &str> {
        if self.players_count() < 3 {
            return Err("too few players");
        }
        for player in &self.players_by_seats {
            match player {
                None => (),
                Some(pl) => if pl.state == PlayerState::NotReady {
                    return Err("not all ready")
                }
            }
        }

        println!("game running, poker module version 1.0.0");

        self.game_state = GamePlayState::Started;

        self.start_round(true);

        Ok(0)
    }

    pub fn start_round(&mut self, first_round: bool) {
        if self.someone_won() {
            self.game_state = GamePlayState::Ended;
            // self.purge_players();
            return;
        }

        self.deal_cards();
        if first_round {
            self.dealer_seat = self.first_taken_seat();
        } else {
            self.dealer_seat = next_player(&self.players_by_seats, self.dealer_seat, self.max_players);
        }
        self.active_player = self.dealer_seat;
        self.set_players_active(true);
        self.reset_players();
        self.set_next_active_player();

        let message = self.player_action(
            self.active_player, 
            PlayerAction::Bet, 
            self.small_blind
        );
        match message {
            0 => {
                    let _ = self.player_action(
                        self.active_player, 
                    PlayerAction::AllIn, 
                    0
                );
            },
            _ => ()
        }

        let message = self.player_action(
            self.active_player, 
            PlayerAction::Bet, 
            self.big_blind
        );
        match message {
            0 => {
                    let _ = self.player_action(
                        self.active_player, 
                    PlayerAction::AllIn, 
                    0
                );
            },
            _ => ()
        }
    }

    pub fn set_next_active_player(&mut self) {
        let mut next_player_seat = next_player(&self.players_by_seats, self.active_player, self.max_players);
        let mut next_player_active: bool = match self.players_by_seats[next_player_seat].as_ref().unwrap().state {
            PlayerState::Folded | PlayerState::Left => false,
            _ => true
        };
        let mut i = 0;
        while !next_player_active {
            next_player_seat = next_player(
                &self.players_by_seats, 
                (self.active_player + i) % self.max_players, 
                self.max_players
            );
            next_player_active = match self.players_by_seats[next_player_seat].as_ref().unwrap().state {
                PlayerState::Folded | PlayerState::Left => false,
                _ => true
            };
            i = i + 1;
            if i > 10 {
                println!("cant find active player!");
                return;
            }
        }
        println!("next active player {}", next_player_seat);
        self.active_player = next_player_seat;
        if self.players_by_seats[self.active_player].unwrap().state == PlayerState::AllIn {
            self.player_action(self.active_player, PlayerAction::AllIn, 0);
        }
    }

    fn set_players_active(&mut self, force: bool) {
        for seat in self.players.values() {
            println!("player state {:?}", self.players_by_seats[*seat].unwrap().state);
            let player = self.players_by_seats[*seat].unwrap();
            self.players_by_seats[*seat] = Some(player.set_active(force));
            println!("player {} state {:?}", player.seat_index, self.players_by_seats[*seat].unwrap().state);
            // if let Some(mut player) = self.players_by_seats[seat] {
            //     println!("setting player {} as active", player.seat_index);
            //     let _ = player.set_active(force);
            //     println!("player state {:?}", player.state);
            // }
            
            // if let Some(player) = self.players_by_seats[*seat] {println!("player state2 {:?}", player.state);}
        }
    }

    fn collect_bets(&mut self) {
        for seat in 0..self.max_players {
            let _ = match &mut self.players_by_seats[seat] {
                Some(pl) => pl.collect_bet(),
                _ => Err("seat empty")
            };
        };
    }

    fn distribute_winnings(&mut self, winner_seat: usize) {
        let winnings: u64 = self.players_by_seats.iter().map(
            |opt_player| match opt_player {
                Some(player) => player.current_bet + player.total_bet,
                None => 0
            }
        ).sum();
        println!("winnings are {}, win seat is {}", winnings, winner_seat);
        for seat in 0..self.max_players {
            match &mut self.players_by_seats[seat] {
                Some(pl) => if pl.seat_index == winner_seat as u8 { pl.collect_win(winnings) },
                _ => ()
            };
        };
    }

    fn someone_won(&self) -> bool {
        let mut non_zero_balance: u8 = 0;
        println!("checking if someone won");
        for seat in 0..self.max_players {
            println!("checking if someone won, seat {}", seat);
            match self.players_by_seats[seat] {
                Some(pl) => if pl.balance > 0 { non_zero_balance += 1; println!("found {} non-zero balance", pl.seat_index); },
                _ => ()
            };
        };  
        println!("checking if someone won, status {}", non_zero_balance <= 2);

        non_zero_balance <= 2
    }

    fn purge_players(&mut self) {
        for seat in 0..self.max_players {
            self.players_by_seats[seat] = None;
            self.players.clear();
        };    
    }

    fn first_taken_seat(&self) -> usize {
        let mut first_taken_seat: usize = 0;
        for (idx, player) in self.players_by_seats.iter().enumerate() {
            match player {
                Some(_) => first_taken_seat = idx,
                _ => ()
            }
        }
        first_taken_seat
    }

    fn reset_players(&mut self) {
        for seat in 0..self.max_players {
            match &mut self.players_by_seats[seat] {
                Some(pl) => pl.reset_for_next_round(),
                _ => ()
            };
        };
    }

    pub fn get_next_active_player(&self) -> usize {
        let mut next_player_seat = next_player(&self.players_by_seats, self.active_player, self.max_players);
        let mut next_player_active: bool = match self.players_by_seats[next_player_seat].as_ref().unwrap().state {
            PlayerState::Folded | PlayerState::Left => false,
            _ => true
        };
        let mut i = 0;
        while !next_player_active {
            next_player_seat = next_player(
                &self.players_by_seats, 
                (self.active_player + i) % self.max_players, 
                self.max_players
            );
            next_player_active = match self.players_by_seats[next_player_seat].as_ref().unwrap().state {
                PlayerState::Folded | PlayerState::Left => false,
                _ => true
            };
            i = i + 1;
            if i > 10 {
                println!("cant find active player!");
                return 0;
            }
        }
        next_player_seat
    }

    fn deal_cards(&mut self) {
        self.shuffle();
        let mut next_card = 0;
        for player in self.players_by_seats.iter_mut() {
            match player {
                Some(pl) => {
                    pl.take_card(0, &self.deck[next_card]);
                    pl.take_card(1, &self.deck[next_card + 1]);
                    next_card += 2;
                }
                None => ()
            }
        }
        for card_offset in 0..5 {
            self.community_cards[card_offset] = Some(self.deck[next_card + card_offset].clone());
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

    fn get_winner_seat(&self) -> usize {
        let mut best_seat = 0;
        let mut best_seat_hand: Option<Eval> = None;
        let community_cards = self.community_cards.map(|card| card.expect("some community cards are None").to_evaluate()).to_vec();
        for seat_id in 0..self.max_players {
            match self.players_by_seats[seat_id] {
                None => (),
                Some(player) => {
                    match player.state {
                        PlayerState::Folded | PlayerState::Left => (),
                        _ => {
                            let private_cards = player.cards.map(|card| card.to_evaluate()).to_vec();
                            let all_cards: Vec<EvaluatorCard> = community_cards.iter().chain(private_cards.iter()).cloned().collect();
                            let current_hand = self.evaluator.evaluate(all_cards);
                            match current_hand {
                                Err(_) => println!("error evaluating hand"),
                                Ok(eval) => {
                                    match best_seat_hand {
                                        None => {
                                            best_seat_hand = Some(eval);
                                            best_seat = player.seat_index;
                                        },
                                        Some(prev_eval) => {
                                            if eval.is_better_than(prev_eval) {
                                                best_seat_hand = Some(eval);
                                                best_seat = player.seat_index;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        best_seat as usize
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
        for player in &self.players_by_seats {
            match player {
                Some(pl) => if pl.current_bet > max_bet {
                    max_bet = pl.current_bet;
                },
                _ => ()
            }
        }
        max_bet
    }

    fn all_bets_even(&self) -> bool {
        let max_bet = self.max_bet();
        for player in &self.players_by_seats {
            match player {
                Some(pl) => match pl.state {
                    PlayerState::Folded | PlayerState::Left => (),
                    _ => if pl.current_bet != max_bet {
                        return false;
                    },
                }
                _ => ()
            }
        }
        true
    }

    fn round_end(&self) -> bool {
        for player in &self.players_by_seats {
            match player {
                Some(pl) => if pl.state == PlayerState::Active {
                    return false;
                },
                _ => ()
            }
        }
        true
    }
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum Color {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

impl Color {
    pub const fn to_evaluate(self) -> EvaluatorColor {
        use Color::*;
        match self {
            Hearts => EvaluatorColor::Hearts,
            Diamonds => EvaluatorColor::Diamonds,
            Spades => EvaluatorColor::Spades,
            Clubs => EvaluatorColor::Clubs
        }
    }
}

#[derive(Clone, Copy, Serialize, Debug)]
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

impl Rank {
    pub const fn to_evaluate(self) -> EvaluatorRank {
        use Rank::*;
        match self {
            Two => EvaluatorRank::Two,
            Three => EvaluatorRank::Three,
            Four => EvaluatorRank::Four,
            Five => EvaluatorRank::Five,
            Six => EvaluatorRank::Six,
            Seven => EvaluatorRank::Seven,
            Eight => EvaluatorRank::Eight,
            Nine => EvaluatorRank::Nine,
            Ten => EvaluatorRank::Ten,
            Jack => EvaluatorRank::Jack,
            Queen => EvaluatorRank::Queen,
            King => EvaluatorRank::King,
            Ace => EvaluatorRank::Ace
        }
    }
}

#[derive(Clone, Copy, Serialize, Debug)]
enum GamePhase {
    PreFlop, // every player has 2 cards, 0 community cards
    Flop, // first 3 community cards
    Turn, // 4th community card
    River // 5th community card on the table
}

#[derive(Clone, Copy, Serialize, Debug)]
pub struct Card {
    rank: Rank,
    color: Color
}

impl Card {
    pub fn new(color: Color, rank: Rank) -> Card {
        Card{color, rank}
    }

    pub fn to_evaluate(&self) -> EvaluatorCard {
        EvaluatorCard::new(self.rank.to_evaluate(), self.color.to_evaluate())
    }
}

#[derive(Clone, Copy, Serialize, Debug, PartialEq)]
pub enum GamePlayState {
    NotStarted,
    Started,
    Ended
}
