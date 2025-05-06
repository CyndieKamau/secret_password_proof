

use std::collections::HashMap;

use crate::simulate_db;

pub struct AppState {
    pub codes: HashMap<&'static str, bool>,
    pub current_input: String,
    pub verdict_message: String,
    pub message: String,
    pub verified: Option<bool>,
    pub done: bool,
}

impl AppState {
    pub fn new(codes: HashMap<&'static str, bool>) -> Self {
        Self {
            codes: simulate_db(),
            current_input: String::new(),
            verdict_message: String::new(),
            message: String::from(""),
            verified: None,
            done: false,
        }
    }

    pub fn check_code(&mut self) {
        let trimmed = self.current_input.trim();
        match self.codes.get(trimmed) {
            Some(true) => {
                self.message = "✅ User is over 18".to_string();
                self.verified = Some(true);
                self.done = true;
            }
            Some(false) => {
                self.message = "🚫 User is under 18".to_string();
                self.verified = Some(false);
                self.done = true;
            }
            None => {
                self.message = "❓ Code not recognized".to_string();
                self.verified = None;
                self.done = true;
            }
        }
    }
}