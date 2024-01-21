use super::utils::create_layout;
use crate::gui::{
    gui_state::GuiState,
    term::Term,
    traits::GuiDisplay,
    widgets::{SnipList, SnipValue},
};
use anyhow::{Ok, Result};
use hms_common::app_dir_client::AppDirClient;
use ratatui::{
    layout::{Direction, Rect},
    widgets::{Block, Paragraph},
};

#[derive(Debug)]
pub struct LargeDisplay {
    term: Term,
    layout: Layout,
}

#[derive(Debug)]
struct Layout {
    snip_list: Rect,
    snip_val: Rect,
    search_bar: Rect,
}

impl Layout {
    fn new(w: u16, h: u16) -> Self {
        let root_rect = Rect::new(5, 1, w.saturating_sub(5), h.saturating_sub(1));

        let root_layout = create_layout(root_rect, Direction::Vertical, vec![90, 5, 5]);
        let main_layout = create_layout(root_layout[0], Direction::Horizontal, vec![35, 65]);

        Self {
            snip_list: main_layout[0],
            snip_val: main_layout[1],
            search_bar: root_layout[1],
        }
    }
}

impl<P> GuiDisplay<P> for LargeDisplay
where
    P: AppDirClient,
{
    fn new() -> Result<Self> {
        let term = Term::start(true)?;
        let size = term.size()?;
        Ok(Self {
            term,
            layout: Layout::new(size.width, size.height),
        })
    }

    fn resize(&mut self, w: u16, h: u16) {
        self.layout = Layout::new(w, h)
    }

    fn update(&mut self, state: &mut GuiState<P>) -> Result<()> {
        let snip_list = SnipList;
        let snip_value = SnipValue::new(state.list_state.selected_snip_value());
        let search_bar =
            Paragraph::new(format!("Search: {}_", state.query())).block(Block::default());
        self.term.draw(|f| {
            f.render_stateful_widget(snip_list, self.layout.snip_list, &mut state.list_state);
            f.render_widget(snip_value, self.layout.snip_val);
            f.render_widget(search_bar, self.layout.search_bar);
        })?;
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        todo!()
    }
}
