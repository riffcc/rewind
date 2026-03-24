//! Rewind — TUI for Replay.

use anyhow::Result;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use rewind::beads;
use rewind::input::{InputEvent, InputManager};

struct App {
    issues: Vec<beads::Issue>,
    list_state: ListState,
    status: String,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            issues: Vec::new(),
            list_state: ListState::default(),
            status: "loading...".into(),
        };
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        match beads::list_issues() {
            Ok(issues) => {
                let count = issues.len();
                self.issues = issues;
                if count > 0 && self.list_state.selected().is_none() {
                    self.list_state.select(Some(0));
                }
                self.status = format!("{count} issues");
            }
            Err(e) => {
                self.status = format!("beads error: {e}");
            }
        }
    }

    fn selected_issue(&self) -> Option<&beads::Issue> {
        self.list_state.selected().and_then(|i| self.issues.get(i))
    }

    fn move_up(&mut self) {
        if self.issues.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        let next = if i == 0 { self.issues.len() - 1 } else { i - 1 };
        self.list_state.select(Some(next));
    }

    fn move_down(&mut self) {
        if self.issues.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        let next = if i >= self.issues.len() - 1 { 0 } else { i + 1 };
        self.list_state.select(Some(next));
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    let mut input = InputManager::new()?;
    let mut app = App::new();

    let result = run(&mut terminal, &mut input, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), LeaveAlternateScreen)?;

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    input: &mut InputManager,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(event) = input.poll()? {
            match event {
                InputEvent::Quit => return Ok(()),
                InputEvent::Up => app.move_up(),
                InputEvent::Down => app.move_down(),
                InputEvent::Select => {
                    if let Some(issue) = app.selected_issue() {
                        app.status = format!("selected: {} — {}", issue.id, issue.title);
                        // TODO: this is where engine::run() gets called
                    }
                }
                InputEvent::Back => {
                    app.refresh();
                    app.status = "refreshed".into();
                }
                _ => {}
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
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
    let header = Paragraph::new(" rewind — replay")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(header, layout[0]);

    // Issue list
    let items: Vec<ListItem> = app
        .issues
        .iter()
        .map(|issue| {
            let status_icon = match issue.status.as_str() {
                "open" => "○",
                "in_progress" => "◐",
                "closed" => "●",
                _ => "?",
            };
            let priority = match issue.priority {
                0 => "P0",
                1 => "P1",
                2 => "P2",
                3 => "P3",
                _ => "P?",
            };
            ListItem::new(format!(
                " {status_icon} [{priority}] {id}  {title}",
                id = issue.id,
                title = issue.title,
            ))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(" issues ({}) ", app.status))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .highlight_style(Style::default().fg(Color::Yellow).bold())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, layout[1], &mut app.list_state);

    // Footer
    let footer = Paragraph::new(" q: quit | ↑↓/jk: navigate | enter: solve | esc: refresh ")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(footer, layout[2]);
}
