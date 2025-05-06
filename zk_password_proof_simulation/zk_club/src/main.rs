mod app;
mod assets;
mod input;
mod ui;


use std::{
    collections::HashMap,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::AppState;
use ui::draw;


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let codes = simulate_db();
    let mut app = AppState::new(codes);


    loop {
        terminal.draw(|f| draw(f, &app))?;

        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        app.current_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.current_input.pop();
                    }
                    KeyCode::Enter => {
                        app.check_code();
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        if app.done {
            // Final draw to show result
            terminal.draw(|f| ui::draw(f, &app))?;
            std::thread::sleep(Duration::from_secs(2));
            break; // Exit the loop — we’ll do shutdown after
        }
    }

    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn simulate_db() -> HashMap<&'static str, bool> {
    let mut map = HashMap::new();
    map.insert("fly87", true);     
    map.insert("wish64", false);   
    map.insert("true99", true);
    map
}