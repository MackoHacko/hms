use anyhow::{Context, Result};
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp,
    },
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::{
    io::{self, stdout, Stdout},
    ops::{Deref, DerefMut},
    sync::OnceLock,
    time::Duration,
};

static ALTERNATE_SCREEN: OnceLock<bool> = OnceLock::new();

#[derive(Debug)]
pub struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn start(alternate_screen: bool) -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        if alternate_screen {
            stdout().execute(EnterAlternateScreen)?;
            ALTERNATE_SCREEN.get_or_init(|| true);
        }
        enable_raw_mode().context("enable raw mode")?;
        Ok(Self { terminal })
    }

    pub fn stop() -> Result<()> {
        let alternate_screen = ALTERNATE_SCREEN.get().map_or(false, |f| *f);
        if alternate_screen {
            stdout().execute(LeaveAlternateScreen)?;
        }
        disable_raw_mode().context("disable raw mode")?;
        Ok(())
    }

    pub fn next_event(timeout: Duration) -> io::Result<Option<Event>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }
        Ok(Some(event::read()?))
    }

    pub fn cursor_position() -> Result<(u16, u16)> {
        let pos = cursor::position().context("cursor position")?;
        Ok(pos)
    }

    pub fn scroll_up(lines: u16) -> Result<()> {
        execute!(stdout(), ScrollUp(lines)).context("scroll up")?;
        Ok(())
    }

    pub fn move_cursor_vertically_to(y: u16) -> Result<()> {
        execute!(stdout(), MoveTo(0, y)).context("move cursor vertically")?;
        Ok(())
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
        let _ = Self::stop();
    }
}
