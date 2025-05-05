//here, we initialize the terminal using ratatui, crossterm
//Prompt the user for the secret password using rpassword (for secrecy)
//Hash and store the user input in AppState
//Start a manual game loop which screen renders (ui.rs), handle the input (input.rs), updates State, exits if game is won or lost




mod app;
mod assets;
mod input;
mod ui;
mod zk;

use crate::app::AppState;
use crate::zk::hash_password;
use crate::input::handle_input;
use std::{time::{Duration, Instant}};

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use rpassword::read_password;



fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("🔐 Enter your secret password (it won’t be shown):");
    io::stdout().flush().unwrap();
    let password = read_password().expect("Failed to read password");
    //let password = "hunter2".to_string();
    let password_hash = hash_password(&password);

    std::process::Command::new("clear").status().unwrap();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

   

    let mut app = AppState::new(password_hash);
    let res = run_game_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("💥 Application error: {}", err);
    }

    Ok(())
}

fn run_game_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        //println!("🌀 Drawing frame...");
        terminal.draw(|f| ui::draw(f, app))?;

        if input::handle_input(app)? {
            break;
        }

        if app.game_over {
            std::thread::sleep(Duration::from_millis(1500));
            break;
        }
    }
    Ok(())
}