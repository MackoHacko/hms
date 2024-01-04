use super::{
    layout::GuiLayout,
    state::GuiState,
    term::Term,
    widgets::{
        gradient_text::{GradientText, LOGO},
        snip_display::SnipDisplay,
        snip_list::SnipList,
    },
};
use anyhow::{Context, Ok, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use hms_common::app_dir_client::AppDirClient;
use hms_config::models::HmsConfig;
use hms_db::{manager::HmsDbManager, models::Snip};
use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph},
};
use std::time::Duration;

#[derive(Debug)]
pub struct Gui {
    term: Term,
    should_quit: bool,
    state: GuiState,
    layout: GuiLayout,
}

impl Gui {
    fn new() -> Result<Self> {
        let term = Term::start()?;
        let state = GuiState::default();
        let layout = GuiLayout::new(term.size()?);
        Ok(Self {
            term,
            should_quit: false,
            state,
            layout,
        })
    }

    pub fn run<T: AppDirClient>(config: HmsConfig, db_manager: HmsDbManager<T>) -> Result<()> {
        install_panic_hook();
        let mut app = Self::new()?;
        let snips = Self::get_snips(&mut app.state, &config, &db_manager)?;
        app.state.list_state.set_snips(snips);
        while !app.should_quit {
            app.draw()?;
            app.handle_events(&config, &db_manager)?;
        }
        Term::stop()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        let state = &mut self.state;
        let q = state.query.to_owned();
        let snip_val = state
            .list_state
            .selected_snip_value()
            .map_or(String::new(), |f| f);
        let alias_list = SnipList;
        let snip_display = SnipDisplay::new(snip_val);
        let logo = GradientText::new(LOGO, (60, 78, 60), (255, 255, 255));

        self.term.draw(|f| {
            f.render_widget(logo, self.layout.logo_area);
            f.render_stateful_widget(
                alias_list,
                self.layout.alias_area,
                &mut self.state.list_state,
            );
            f.render_widget(snip_display, self.layout.snip_area);
            f.render_widget(
                Paragraph::new(format!("Search: {}_", q)).block(Block::default()),
                self.layout.search_area,
            )
        })?;
        Ok(())
    }

    fn handle_events<T: AppDirClient>(
        &mut self,
        config: &HmsConfig,
        db_manager: &HmsDbManager<T>,
    ) -> Result<()> {
        match Term::next_event(Duration::from_millis(32))? {
            Some(Event::Key(key)) => self.handle_key_event(key, &config, &db_manager),
            Some(Event::Resize(width, height)) => self.resize(width, height),
            _ => Ok(()),
        }
    }

    fn resize(&mut self, width: u16, height: u16) -> Result<()> {
        let new_area = Rect::new(0, 0, width, height);
        self.layout = GuiLayout::new(new_area);
        Ok(self.term.resize(new_area)?)
    }

    fn handle_key_event<T: AppDirClient>(
        &mut self,
        key: KeyEvent,
        config: &HmsConfig,
        db_manager: &HmsDbManager<T>,
    ) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        let state = &mut self.state;
        match key.code {
            KeyCode::Esc | KeyCode::Char('c') => {
                if key.code == KeyCode::Esc || key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.should_quit = true;
                }
            }
            KeyCode::Up => {
                state
                    .list_state
                    .select(state.list_state.selected_snip_index().saturating_sub(1));
            }
            KeyCode::Down => {
                state
                    .list_state
                    .select(state.list_state.selected_snip_index().saturating_add(1));
            }
            KeyCode::Char(c) => {
                state.query.push(c);
                let snips = Self::get_snips(state, config, db_manager)?;
                self.state.list_state.set_snips(snips)
            }
            KeyCode::Backspace => {
                state.query.pop();
                let snips = Self::get_snips(state, config, db_manager)?;
                self.state.list_state.set_snips(snips)
            }
            KeyCode::Enter => {
                if let Some(data) = state.list_state.selected_snip_value() {
                    let _ = cli_clipboard::set_contents(data);
                }
                self.should_quit = true;
            }
            _ => {}
        };
        Ok(())
    }

    fn get_snips<T: AppDirClient>(
        state: &mut GuiState,
        config: &HmsConfig,
        db_manager: &HmsDbManager<T>,
    ) -> Result<Vec<Snip>> {
        db_manager
            .with_db(|db| db.find_snip_by_alias(&state.query, config.snip_limit))
            .context("db.find_snip_by_alias")
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
