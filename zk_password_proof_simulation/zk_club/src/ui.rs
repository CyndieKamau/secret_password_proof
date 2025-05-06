use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::assets::{WELCOME_BANNER, BOUNCER_ART_ONE, BOUNCER_ART_TWO};

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),   // Welcome Banner
            Constraint::Length(3),   // Prompt
            Constraint::Length(3),   // Input
            Constraint::Min(3),      // Result/Feedback
        ])
        .split(f.area());

    let banner = Paragraph::new(Text::from(WELCOME_BANNER))
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("🎉 ZK CLUB"));
    f.render_widget(banner, chunks[0]);


    let prompt = Paragraph::new("Enter your secret club code:")
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Bouncer"));
    f.render_widget(prompt, chunks[1]);

    let input = Paragraph::new(app.current_input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Your Code"));
    f.render_widget(input, chunks[2]);

    let result_color = match app.verified {
        Some(true) => Color::Green,
        Some(false) => Color::Red,
        None => Color::Yellow,
    };

    let verdict_display = format!("{}\n{}\n{}",
        app.message, 
        "=======================================",
        BOUNCER_ART_TWO);

    let result = Paragraph::new(verdict_display)
        .style(Style::default().fg(result_color))
        .block(Block::default().borders(Borders::ALL).title("Verification Result"));

    f.render_widget(result, chunks[3]);
}