use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::assets::{DINO_ART, DINO_JUMP_ART, CACTUS_ART, GAME_OVER_ART, WINNER_ART, WELCOME_BANNER, CACTUS_ART_TWO, DINO_ART_TWO};

pub fn draw(f: &mut Frame, app: &AppState) {
    let size = f.area();
    //println!("{:?}", f.area()); 

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),   // Welcome banner
            Constraint::Min(10),     // Game canvas
            Constraint::Length(8),   // Input line
        ])
        .split(size);

  
    let banner = Paragraph::new(Text::from(WELCOME_BANNER))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("ZK Dino 🦖"));

    f.render_widget(banner, chunks[0]);

    let game_canvas = if app.success {
        Paragraph::new(Text::from(WINNER_ART))
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("🎉 YOU WIN"))
    } else if app.game_over {
        Paragraph::new(Text::from(GAME_OVER_ART))
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("💀 GAME OVER"))
    } else {
        let game_frame = build_game_frame(app);
        Paragraph::new(Text::from(game_frame))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("🦴 The Terminal Desert"))
    };
    f.render_widget(game_canvas, chunks[1]);

   
    let input = Paragraph::new(Text::from(format!("Your guess: {}", app.current_guess)))
        .style(Style::default().fg(Color::Yellow))
         .block(Block::default().borders(Borders::ALL).title("Password Guess"));

    f.render_widget(input, chunks[2]);
}

// fn build_game_frame(app: &AppState) -> String {
//     if app.success {
//         return WINNER_ART.to_string();
//     }

//     if app.game_over {
//         return GAME_OVER_ART.to_string();
//     }

//     // Select dino frame (can alternate if you like)
//     let dino = if app.attempts_left % 2 == 0 {
//         DINO_JUMP_ART
//     } else {
//         DINO_ART
//     };

//     // Dynamic cactus spacing
//     let spacing = match app.attempts_left {
//         5 => "         ",
//         4 => "      ",
//         3 => "   ",
//         2 => " ",
//         1 => "",
//         _ => "",
//     };

//     // Final scene frame
//     format!(
//         "{dino}\n\n{spacing}{CACTUS_ART}",
//         dino = dino,
//         spacing = spacing,
//         CACTUS_ART = CACTUS_ART
//     )
// }

fn build_game_frame(app: &AppState) -> String {
    if app.success {
        return WINNER_ART.to_string();
    }

    if app.game_over {
        return GAME_OVER_ART.to_string();
    }

    let dino_lines: Vec<&str> = DINO_ART.lines().collect();
    let cactus_lines: Vec<&str> = CACTUS_ART.lines().collect();

   
    let spacing: usize = match app.attempts_left {
        5 => 20,
        4 => 16,
        3 => 12,
        2 => 8,
        1 => 4,
        _ => 0,
    };

    let dino_height = dino_lines.len();
    let cactus_height = cactus_lines.len();
    let total_lines = dino_height.max(cactus_height);

    let mut combined = String::new();

    for i in 0..total_lines {
        let dino_line = if i < dino_lines.len() {
            dino_lines[i]
        } else {
            ""
        };

        let cactus_line = if i < cactus_lines.len() {
            cactus_lines[i]
        } else {
            ""
        };

        let padding = " ".repeat(spacing);
        let line = format!("{dino}{pad}{cactus}\n", dino = dino_line, pad = padding, cactus = cactus_line);
        combined.push_str(&line);
    }


    combined
}