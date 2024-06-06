use crate::poker::game::{Card, Color, Rank};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct Player {
    pub seat_index: u8,
    pub balance: u64,
    pub current_bet: u64,
    pub total_bet: u64,
    pub state: PlayerState,
    appearance_type: u8,
    pub cards: [Card; 2],
}

impl Player {
    pub fn new_player(seat_index: u8, balance: u64, appearance_type: u8) -> Player {
        let cards: [Card; 2] = [
            // dummy values replaced later
            Card::new(Color::Spades, Rank::Two),
            Card::new(Color::Spades, Rank::Two),
        ];
        Player {
            seat_index,
            balance,
            current_bet: 0,
            total_bet: 0,
            state: PlayerState::NotReady,
            appearance_type,
            cards,
        }
    }

    pub fn set_ready(&mut self, ready: bool) {
        if ready {
            self.state = PlayerState::Ready;
        } else {
            self.state = PlayerState::NotReady;
        }
    }

    pub fn perform_action(&mut self, action: PlayerAction, amount: u64) -> Result<u64, &str> {
        match self.state {
            PlayerState::Folded | PlayerState::Left => return Err("player is unable to take an action"),
            _ => ()
        }
        match action {
            PlayerAction::Bet => self.bet(amount), // amount is how much money to bet
            PlayerAction::AllIn => self.all_in(),
            PlayerAction::Call => self.call(amount), // amount is the bet that is to be called
            PlayerAction::Check => self.check(amount), // amount is previous bet that is to be equalized
            PlayerAction::Fold => self.fold(),
        }
    }

    pub fn take_card(&mut self, which: usize, card: &Card) {
        // which is index, 0 or 1
        self.cards[which] = card.clone();
        println!("player at seat {} took card {:?} into idx {}", self.seat_index, card, which);
    }

    pub fn collect_win(&mut self, amount: u64) {
        self.balance += amount;
        println!("player at seat {} took winnings {}", self.seat_index, amount);
    }

    pub fn reset_for_next_round(&mut self) {
        self.current_bet = 0;
        self.total_bet = 0;
    }

    fn check(&mut self, to_check: u64) -> Result<u64, &str> {
        // when betting more money is not needed
        if self.current_bet < to_check {
            println!("player at seat {} failed to check with {}", self.seat_index, to_check);
            return Err("bet amount too small to check");
        }
        self.state = PlayerState::Check;
        println!("player at seat {} checked sum of {} with {}", self.seat_index, to_check, self.current_bet);
        Ok(self.current_bet)
    }

    fn call(&mut self, to_call_total: u64) -> Result<u64, &str> {
        // when betting more money is needed to match previous players bet
        let amount = to_call_total - self.current_bet;

        if amount > self.balance {
            println!("player at seat {} failed to call with {}", self.seat_index, to_call_total);
            return Err("insufficient balance");
        }

        self.balance -= amount;
        self.current_bet += amount;

        self.state = PlayerState::Call;
        println!("player at seat {} called with {}", self.seat_index, to_call_total);

        if self.balance == 0 {
            self.state = PlayerState::AllIn;
            println!("player at seat {} called but thats all in", self.seat_index);
        }

        Ok(self.current_bet)
    }

    fn bet(&mut self, amount: u64) -> Result<u64, &str> {
        // also as rise (bet is when you are going first, rise if you aren't first)
        if amount > self.balance {
            println!("player at seat {} failed to bet with {}", self.seat_index, amount);
            return Err("insufficient balance");
        }
        self.balance -= amount;
        self.current_bet += amount;
        self.state = PlayerState::Active;
        println!("player at seat {} bet successfully with {}", self.seat_index, amount);

        if self.balance == 0 {
            self.state = PlayerState::AllIn;
            println!("player at seat {} bet but thats all in", self.seat_index);
        }

        Ok(self.current_bet)
    }

    fn all_in(&mut self) -> Result<u64, &str> {
        self.current_bet += self.balance;
        self.balance = 0;
        self.state = PlayerState::AllIn;
        println!("player at seat {} all in", self.seat_index);
        Ok(self.current_bet)
    }

    fn fold(&mut self) -> Result<u64, &str> {
        // return type for completness sake
        self.state = PlayerState::Folded;
        println!("player at seat {} folded", self.seat_index);
        Ok(self.balance)
    }

    pub fn set_active(mut self, force: bool) -> Self {
        match self.state {
            PlayerState::Folded | PlayerState::Left | PlayerState::AllIn => {
                if !force { return self; }
            },
            _ => ()
        };
        self.state = PlayerState::Active;
        println!("setting player state at seat {} as active, {:?}", self.seat_index, self.state);
        self
    }

    pub fn collect_bet(&mut self) -> Result<u64, &str> {
        self.total_bet += self.current_bet;
        self.current_bet = 0;
        println!("player at seat {} collected his {} bet into the main pool", self.seat_index, self.total_bet);
        Ok(self.total_bet)
    }
}

#[derive(Serialize)]
pub struct PlayerData {
    pub seat_index: u8,
    pub balance: u64,
    pub state: PlayerState,
    pub bet_amount: u64,
    pub nickname: String,
}

#[derive(PartialEq, Clone, Copy, Serialize, Debug)]
pub enum PlayerState {
    Ready,    // only before game begins
    NotReady, // only before game begins
    Active,   // when player raised or waits for his turn
    Check,    // when player checked
    Call,     // when player called
    AllIn,    // when player all-ined
    Folded,   // when player folded his current hand
    Left,     // when player left the game
}

#[derive(PartialEq, Deserialize, Clone, Copy, Serialize)]
pub enum PlayerAction {
    Check,
    Call,
    Bet,
    Fold,
    AllIn,
}
