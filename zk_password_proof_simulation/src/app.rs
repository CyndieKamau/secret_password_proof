//here, we manage the secret hash, current guess, no of remaining attempts, whether you've won the game or lost

use crate::zk::hash_password;

pub struct AppState {
    pub secret_hash: String,
    pub current_guess: String,
    pub attempts_left: u8,
    pub game_over: bool,
    pub success: bool,
}

impl AppState {
    pub fn new(secret_hash: String) -> Self {
        Self {
            secret_hash,
            current_guess: String::new(),
            attempts_left: 5,
            game_over: false,
            success: false,
        }
    }

    pub fn submit_guess(&mut self) {
        let guess_hash = hash_password(&self.current_guess);

        if guess_hash == self.secret_hash {
            self.success = true;
            self.game_over = true;
        } else {
            self.attempts_left = self.attempts_left.saturating_sub(1);

            if self.attempts_left == 0 {
                self.success = false;
                self.game_over = true;
            }
        }

        self.current_guess.clear();
    }
}