use super::{gui_state::GuiState, term::Term, traits::GuiDisplay};
use anyhow::{Ok, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use hms_common::app_dir_client::AppDirClient;
use hms_config::models::HmsConfig;
use hms_db::manager::HmsDbManager;
use std::time::Duration;

#[derive(Debug)]
pub struct Gui<'a, D, P>
where
    D: GuiDisplay<P>,
    P: AppDirClient,
{
    gui_state: GuiState<'a, P>,
    display: D,
    should_quit: bool,
}

impl<'a, D, P> Gui<'a, D, P>
where
    D: GuiDisplay<P>,
    P: AppDirClient,
{
    fn new(db_manager: &'a HmsDbManager<'a, P>, cfg: HmsConfig) -> Result<Self> {
        Ok(Self {
            gui_state: GuiState::new(db_manager, cfg.snip_limit)?,
            display: D::new()?,
            should_quit: false,
        })
    }

    pub fn run(db_manager: &'a HmsDbManager<'a, P>, cfg: HmsConfig) -> Result<()> {
        install_panic_hook();
        let mut gui = Self::new(db_manager, cfg)?;

        while !gui.should_quit {
            gui.display.update(&mut gui.gui_state)?;
            match Term::next_event(Duration::from_millis(32))? {
                Some(Event::Resize(width, height)) => {
                    gui.display.resize(width, height);
                }
                Some(Event::Key(key_event)) => gui.handle_key_event(key_event)?,
                _ => {}
            };
        }
        Term::stop()?;
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Esc | KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.code == KeyCode::Esc
                    || key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    self.should_quit = true;
                }
            }
            KeyCode::Up => {
                self.gui_state.list_state.select(
                    self.gui_state
                        .list_state
                        .selected_snip_index()
                        .saturating_sub(1),
                );
            }
            KeyCode::Down => {
                self.gui_state.list_state.select(
                    self.gui_state
                        .list_state
                        .selected_snip_index()
                        .saturating_add(1),
                );
                self.gui_state.paginate()?;
            }
            KeyCode::Char(c) => {
                self.gui_state.append_query(c)?;
            }
            KeyCode::Backspace => {
                self.gui_state.pop_query()?;
            }
            _ => {}
        }
        Ok(())
    }
}

fn install_panic_hook() {
    better_panic::install();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Term::stop();
        hook(info);
        std::process::exit(1);
    }));
}
