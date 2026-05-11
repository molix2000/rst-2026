use color_eyre::Result;
use crossterm::event::{self, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::DefaultTerminal;
use std::io::{self, Write};
pub fn rata() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    enable_raw_mode()?;
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = ratatui::widgets::Block::default()
                .title("Hello, World!")
                .borders(ratatui::widgets::Borders::ALL);
            f.render_widget(block, size);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}
