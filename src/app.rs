// Application State and Event Loop

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppScreen{
  Main,  // the main data view - ascii, hex, and input
}

// main application state 
pub struct App {
  quit_initiated: bool,
  current_view: AppScreen
}

// initialize a new app with the default state
impl App {
  pub fn new() -> Self {
    quit_initiated: false,
    current_view: AppScreen::Main
  }
}


// running the main event loop
pub fn run(&mut self, terminal: &mut crate::tui::Terminal) -> Result<()> {

  // while the user hasn't initiated quitting the application
  while !self.quit_initiated {
    // 1. render the current UI state to the terminal
    let screen = self.current_view
    terminal.draw(|frame| {
      Self::render(frame, screen)
    })?;

    // 2. listen for user input
    let event = event::read()?;

    // 3. process input and update app state
    self.handle_event(event)
  }
}

