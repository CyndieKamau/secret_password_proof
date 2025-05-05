use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::assets::{DINO_ART, DINO_JUMP_ART, CACTUS_ART, GAME_OVER_ART, WINNER_ART, WELCOME_BANNER};

pub fn draw(f: &mut Frame, app: &AppState) {
    let size = f.area();
    //println!("{:?}", f.area()); 

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),   // Welcome banner
            Constraint::Min(10),     // Game canvas
            Constraint::Length(3),   // Input line
        ])
        .split(size);

    // 🏁 Top Banner
    let banner = Paragraph::new("Banner Test")
        .block(Block::default().borders(Borders::ALL).title("Banner"));

    f.render_widget(banner, chunks[0]);

    // 🌵 Game Scene
    let game_frame = build_game_frame(app);
    let game_canvas = Paragraph::new("Game Frame Here")
        .block(Block::default().borders(Borders::ALL).title("Game"));
    f.render_widget(game_canvas, chunks[1]);

    // ⌨️ Guess Box
    let input = Paragraph::new(Text::from(format!("Your guess: {}", app.current_guess)))
        .style(Style::default().fg(Color::Yellow))
         .block(Block::default().borders(Borders::ALL).title("Password Guess"));

    f.render_widget(input, chunks[2]);
}

fn build_game_frame(app: &AppState) -> String {
    if app.success {
        return WINNER_ART.to_string();
    }

    if app.game_over {
        return GAME_OVER_ART.to_string();
    }

    // Select dino frame (can alternate if you like)
    let dino = if app.attempts_left % 2 == 0 {
        DINO_JUMP_ART
    } else {
        DINO_ART
    };

    // Dynamic cactus spacing
    let spacing = match app.attempts_left {
        5 => "         ",
        4 => "      ",
        3 => "   ",
        2 => " ",
        1 => "",
        _ => "",
    };

    // Final scene frame
    format!(
        "{dino}\n\n{spacing}{CACTUS_ART}",
        dino = dino,
        spacing = spacing,
        CACTUS_ART = CACTUS_ART
    )
}

// use ratatui::{
//     backend::Backend,
//     Frame,
//     widgets::{Paragraph, Block, Borders},
//     layout::{Layout, Constraint, Direction},
//     text::Text,
// };

// use crate::app::AppState;

// pub fn draw(f: &mut Frame, _app: &AppState) {
//     let size = f.area();

//     // Render a simple "Hello World" block
//     let block = Paragraph::new(Text::from("Hello from Dino"))
//         .block(Block::default().borders(Borders::ALL).title("ZK Dino UI Test"));

//     f.render_widget(block, size);
// }