use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::collections::HashMap;

use crate::app::AppState;

pub fn handle_input(app: &mut AppState, codes: &HashMap<&str, bool>) -> std::io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char(c) => {
                    app.current_input.push(c);
                }
                KeyCode::Backspace => {
                    app.current_input.pop();
                }
                KeyCode::Enter => {
                    let code = app.current_input.trim();
                    match codes.get(code) {
                        Some(true) => {
                            app.verified = Some(true);
                            app.message = "✅ Access Granted: User is over 18".to_string();
                        }
                        Some(false) => {
                            app.verified = Some(false);
                            app.message = "⛔ Access Denied: Underage user!".to_string();
                        }
                        None => {
                            app.verified = Some(false);
                            app.message = "❓ Code not recognized.".to_string();
                        }
                    }

                    // Optional: keep code on screen or reset
                    // app.current_input.clear(); // if you want to clear for retry
                }
                KeyCode::Esc => return Ok(true), // Exit game
                _ => {}
            }
        }
    }

    Ok(false)
}