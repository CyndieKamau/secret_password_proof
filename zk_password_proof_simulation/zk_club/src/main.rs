use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal, Frame,
    style::{Style, Color},
    text::Text,
};
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute};
use std::{io, collections::HashMap, thread::sleep, time::Duration};


fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let codes = simulate_db();
    let code = "fly87";

    terminal.draw(|f| draw_ui(f, &codes, code))?;

    sleep(Duration::from_secs(3));

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn simulate_db() -> HashMap<&'static str, bool> {
    let mut map = HashMap::new();
    map.insert("fly87", true);     // over 18
    map.insert("wish64", false);   // underage
    map.insert("true99", true);
    map
}

fn draw_ui(f: &mut Frame, codes: &HashMap<&str, bool>, code: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5)])
        .split(f.area());

    let verdict = match codes.get(code) {
        Some(true) => "✅ User is over 18",
        Some(false) => "🚫 User is under 18",
        None => "❓ Code not recognized",
    };

    let paragraph = Paragraph::new(Text::from(verdict))
        .block(Block::default().borders(Borders::ALL).title("Club Bouncer"))
        .style(Style::default().fg(Color::Green));

    f.render_widget(paragraph, chunks[0]);
}
