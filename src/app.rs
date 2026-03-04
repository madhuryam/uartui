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

}