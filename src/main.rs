mod tui;

use anyhow::Result;
use crate::tui::Terminal;

/// entry point for uartui
fn main() -> Result<()> {
    // creating terminal wrapper (w error propogation)
    let mut terminal = Terminal::new()?;

    // event loop: draw welcome screen, wait for user input, repeat till user types 'q'
    loop {
        // Rust closure - capturing frame
        terminal.draw(|frame| {
            use ratatui::prelude::*;
            use ratatui::widgets::{Block, Paragraph};

            // creating bordered block as a container
            let block = Block::bordered().title(" uartui ");

            // creating paragraph with welcome text
            let text = vec![
                Line::from("Welcome to uartui — a modern serial port TUI"),
                Line::from(""),
                Line::from("Press 'q' to quit"),
                Line::from(""),
                Line::from(format!("Version: {}", env!("CARGO_PKG_VERSION"))),
            ];
            let paragraph = Paragraph::new(text).block(block);

            // render paragraph to fill entire terminal area
            frame.render_widget(paragraph, frame.area());
        })?;

        // wait for key event (note: read call is currently blocking, todo: switch to async event handling)
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {

            if key.code == crossterm::event::KeyCode::Char('q') {
                break;
            }
        }
    }

    // automatic terminal cleanup w drop trait implementation (restore())
    Ok(())
}