pub struct Player {
    pub seat_index: u8,
    balance: u64,
    bet_amount: u64,
    state: PlayerState,
    appearance_type: u8,
}

impl Player {
    pub fn new_player(
        seat_index: u8, 
        balance: u64, 
        appearance_type: u8
    ) -> Player {
            Player{seat_index, balance, bet_amount: 0, state: PlayerState::NotReady, appearance_type}
    }

    pub fn set_ready(&mut self, ready: bool) {
        if ready {
            self.state = PlayerState::Ready
        } else {
            self.state = PlayerState::NotReady
        }
    }
    
    pub fn check(&self, to_check: u64) -> Result<u64, &str> { // when betting more money is not needed
        if self.bet_amount < to_check {
            return Err("bet amount too small to check");
        }
        Ok(self.bet_amount)
    }
    
    pub fn call(&mut self, to_call_total: u64) -> Result<u64, &str> { // when betting more money is needed to match previous players bet
        self.bet(to_call_total - self.bet_amount)
    }

    pub fn bet(&mut self, amount: u64) -> Result<u64, &str> { // also as rise (bet is when you are going first, rise if you aren't first)
        if amount > self.balance {
            return Err("insufficient balance");
        }
        if self.state != PlayerState::Active {
            return Err("player is not able to bet");
        }
        self.balance -= amount;
        self.bet_amount += amount;

        if self.balance == 0 {
            self.state = PlayerState::AllIn;
        }

        Ok(self.bet_amount)
    }

    pub fn fold(&mut self) -> Result<u64, &str> { // return type for completness sake
        self.state = PlayerState::Folded;
        Ok(self.balance)
    }
}

#[derive(PartialEq)]
enum PlayerState {
    Ready, // only before game begins
    NotReady, // only before game begins
    Active, // when player checked, bet or raised
    AllIn, // when player all-ined
    Folded, // when player folded his current hand
    Left, // when player left the game
}
