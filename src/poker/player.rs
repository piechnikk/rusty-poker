use crate::poker::game::{Card, Color, Rank};
use serde::Serialize;

#[derive(Clone)]
pub struct Player {
    pub seat_index: u8,
    balance: u64,
    pub current_bet: u64,
    total_bet: u64,
    pub state: PlayerState,
    appearance_type: u8,
    cards: [Card; 2]
}

impl Player {
    pub fn new_player(
        seat_index: u8, 
        balance: u64, 
        appearance_type: u8
    ) -> Player {
        let cards: [Card; 2] = [ // dummy values replaced later
            Card::new(Color::Spades, Rank::Two),
            Card::new(Color::Spades, Rank::Two),
        ];
        Player{seat_index, balance, current_bet: 0, total_bet: 0, state: PlayerState::NotReady, appearance_type, cards}
    }

    pub fn set_ready(&mut self, ready: bool) {
        if ready {
            self.state = PlayerState::Ready
        } else {
            self.state = PlayerState::NotReady
        }
    }

    pub fn perform_action(&mut self, action: PlayerAction, amount: u64) -> Result<u64, &str> {
        match action {
            PlayerAction::Bet => self.bet(amount), // amount is how much money to bet
            PlayerAction::AllIn => self.all_in(),
            PlayerAction::Call => self.bet(amount - self.current_bet), // amount is the beet that is to be called
            PlayerAction::Check => self.check(amount), // amount is previous bet that is to be equalized
            PlayerAction::Fold => self.fold()
        }
    }

    pub fn take_card(&mut self, which: usize, card: &Card) { // which is index, 0 or 1
        self.cards[which] = card.clone();
    }
    
    fn check(&self, to_check: u64) -> Result<u64, &str> { // when betting more money is not needed
        if self.current_bet < to_check {
            return Err("bet amount too small to check");
        }
        Ok(self.current_bet)
    }
    
    fn call(&mut self, to_call_total: u64) -> Result<u64, &str> { // when betting more money is needed to match previous players bet
        self.bet(to_call_total - self.current_bet)
    }

    fn bet(&mut self, amount: u64) -> Result<u64, &str> { // also as rise (bet is when you are going first, rise if you aren't first)
        if amount > self.balance {
            return Err("insufficient balance");
        }
        if self.state != PlayerState::Active {
            return Err("player is not able to bet");
        }
        self.balance -= amount;
        self.current_bet += amount;

        if self.balance == 0 {
            self.state = PlayerState::AllIn;
        }

        Ok(self.current_bet)
    }
    
    fn all_in(&mut self) -> Result<u64, &str> {
        self.current_bet += self.balance;
        self.balance = 0;
        Ok(self.current_bet)
    }

    fn fold(&mut self) -> Result<u64, &str> { // return type for completness sake
        self.state = PlayerState::Folded;
        Ok(self.balance)
    }

    fn collect_bet(&mut self) -> Result<u64, &str> {
        self.total_bet += self.current_bet;
        self.current_bet = 0;
        Ok(self.total_bet)
    }
}

#[derive(PartialEq, Clone)]
pub enum PlayerState {
    Ready, // only before game begins
    NotReady, // only before game begins
    Active, // when player checked, bet or raised
    AllIn, // when player all-ined
    Folded, // when player folded his current hand
    Left, // when player left the game
}

#[derive(PartialEq)]
pub enum PlayerAction {
    Check,
    Call,
    Bet,
    Fold,
    AllIn
}
