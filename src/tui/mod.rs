#![cfg(feature = "tui")]

use anyhow::Result;
use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Stdout};

pub fn run(_theme: Option<String>, _theme_file: Option<std::path::PathBuf>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // minimal loop: draw a frame with instructions and exit on 'q'
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("pdf-ops TUI (press q to quit)")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') { break; }
            }
        }
    }

    disable_raw_mode()?;
    // leave alt screen
    // SAFETY: We created a stdout above, but need a fresh one to leave
    let mut out = std::io::stdout();
    execute!(out, LeaveAlternateScreen)?;
    Ok(())
}

