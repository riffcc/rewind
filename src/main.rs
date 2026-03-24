//! Rewind — TUI for Replay.

use anyhow::Result;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use rewind::input::{InputEvent, InputManager};

fn main() -> Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    let mut input = InputManager::new()?;
    let result = run(&mut terminal, &mut input);

    disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    input: &mut InputManager,
) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame))?;

        if let Some(event) = input.poll()? {
            match event {
                InputEvent::Quit => return Ok(()),
                _ => {}
            }
        }
    }
}

fn ui(frame: &mut Frame) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new(" rewind — replay observer")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(header, layout[0]);

    // Main content
    let body = Paragraph::new("No replay session active.\n\nPress 'q' to quit.")
        .block(
            Block::default()
                .title(" status ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(body, layout[1]);

    // Footer
    let footer = Paragraph::new(" q: quit | arrows/hjkl: navigate | enter: select | esc: back | gamepad: connected ")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, layout[2]);
}
