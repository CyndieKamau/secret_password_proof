// use crossterm::event::{self, Event, KeyCode, KeyEventKind};
// use std::io;
// use crate::app::AppState;

// pub fn handle_input(app: &mut AppState) -> io::Result<bool> {
//     if event::poll(std::time::Duration::from_millis(100))? {
//         if let Event::Key(key_event) = event::read()? {
//             if key_event.kind == KeyEventKind::Press {
//                 match key_event.code {
//                     KeyCode::Char('q') => return Ok(true), // Quit
//                     KeyCode::Enter => {
//                         app.submit_guess(); // Check the guess
//                     },
//                     KeyCode::Char(c) => {
//                         app.current_guess.push(c);
//                     },
//                     KeyCode::Backspace => {
//                         app.current_guess.pop();
//                     },
//                     _ => {}
//                 }
//             }
//         }
//     }
//     Ok(false)
// }

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crate::app::AppState;

pub fn handle_input(app: &mut AppState) -> std::io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(150))? {
        let evt = event::read()?;
        if let Event::Key(KeyEvent { code, .. }) = evt {
            match code {
                KeyCode::Char(c) => {
                    app.current_guess.push(c);
                    println!("Typed: {}", c); // Debug
                }
                KeyCode::Backspace => {
                    app.current_guess.pop();
                    println!("Backspace");
                }
                KeyCode::Enter => {
                    let guess = app.current_guess.trim();
                    if guess.is_empty() {
                        return Ok(false);
                    }

                    let hashed = crate::zk::hash_password(guess);
                    if hashed == app.secret_hash {
                        app.success = true;
                        app.game_over = true;
                        println!("✅ Password matched!");
                    } else {
                        app.attempts_left -= 1;
                        if app.attempts_left == 0 {
                            app.game_over = true;
                        }
                        println!("❌ Wrong guess: {}", guess);
                    }

                    app.current_guess.clear();
                }
                KeyCode::Esc => return Ok(true),
                _ => {}
            }
        }
    }

    Ok(false)
}