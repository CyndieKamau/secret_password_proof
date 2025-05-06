use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::app::AppState;

pub fn handle_input(app: &mut AppState) -> std::io::Result<bool> {
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
                    app.check_code();
                }
                KeyCode::Esc => return Ok(true), // Exit
                _ => {}
            }
        }
    }

    Ok(false)
}