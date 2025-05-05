pub struct AppState {
    pub current_input: String,
    pub verified: Option<bool>, 
    pub message: String,        // Optional feedback from bouncer
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            verified: None,
            message: String::from("👮 Bouncer: Are you over 18? Enter your code:"),
        }
    }

    pub fn reset(&mut self) {
        self.current_input.clear();
        self.verified = None;
        self.message = String::from("👮 Bouncer: Are you over 18? Enter your code:");
    }
}