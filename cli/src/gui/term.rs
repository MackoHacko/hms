use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::{
    io::{self, stdout, Stdout},
    ops::{Deref, DerefMut},
    time::Duration,
};

#[derive(Debug)]
pub struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn start() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode().context("enable raw mode")?;
        Ok(Self { terminal })
    }

    pub fn stop() -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode().context("disable raw mode")?;
        Ok(())
    }

    pub fn next_event(timeout: Duration) -> io::Result<Option<Event>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }
        let event = event::read()?;
        Ok(Some(event))
    }
}

impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = Term::stop();
    }
}
