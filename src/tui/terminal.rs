// tui/terminal.rs — Terminal wrapper with RAII (Resource Acquisition Is Initialization) cleanup

use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;
use std::io::{self, stdout, Stdout};

// creating type alias for ratatui Terminal
type RatatuiTerminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub struct Terminal {
    // the underlying ratatui terminal instance of the app
    inner: RatatuiTerminal,
}
/// creating new terminal, entering raw mode and the alternate screen 
impl Terminal {
    pub fn new() -> Result<Self> {
      // raw mode: disables line buffering, echo, and special key handling
      // sends key presses as raw bytes
      enable_raw_mode().context("Failed to enable raw terminal mode")?;

      // entering alternate screen, preserving previous user terminal context
      execute!(stdout(), EnterAlternateScreen)
          .context("Failed to enter alternate screen")?;

      // creating ratatui terminal with crossterm backend
      let backend = CrosstermBackend::new(stdout());
      let inner = ratatui::Terminal::new(backend)
          .context("Failed to create ratatui terminal")?;

      Ok(Self { inner })
    }

    /// drawing a frame to the terminal
    pub fn draw(&mut self, render_callback: impl FnOnce(&mut ratatui::Frame)) -> Result<()> {
        self.inner
            .draw(render_callback)
            .context("Failed to draw frame")?;
        Ok(())
    }

    /// restoring the terminal to its original state
    fn restore(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

// destructor (drop trait) for terminal
impl Drop for Terminal {
    fn drop(&mut self) {
        self.restore();
    }
}
