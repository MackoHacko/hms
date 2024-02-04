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
    widgets::{Block, Clear, Paragraph, Widget},
};

#[derive(Debug)]
struct Layout {
    root: Rect,
    snip_list: Rect,
    snip_val: Rect,
    search_bar: Rect,
}

impl Layout {
    fn new(y: u16, w: u16, h: u16) -> Self {
        let root_rect = Rect::new(0, y, w, h);

        let root_layout = create_layout(root_rect, Direction::Vertical, vec![95, 5]);
        let main_layout = create_layout(root_layout[0], Direction::Horizontal, vec![20, 80]);

        Self {
            root: root_rect,
            snip_list: main_layout[0],
            snip_val: main_layout[1],
            search_bar: root_layout[1],
        }
    }
}

#[derive(Debug)]
pub struct SmallDisplay {
    term: Term,
    layout: Layout,
    cursor_start: (u16, u16),
}

impl SmallDisplay {
    fn height(term_height: u16) -> u16 {
        (term_height as f32 * 0.4).round() as u16
    }

    fn calculate_and_prepare_cursor_y_start(
        term_height: u16,
        cursor_y: u16,
        req_height: u16,
    ) -> Result<u16> {
        let available_height = term_height.saturating_sub(cursor_y);
        if available_height < req_height {
            let scroll_lines = req_height - available_height;
            Term::scroll_up(scroll_lines)?;
            let y = cursor_y.saturating_sub(scroll_lines);
            Term::move_cursor_vertically_to(y)?;
            Ok(y)
        } else {
            Ok(cursor_y)
        }
    }
}

impl<P> GuiDisplay<P> for SmallDisplay
where
    P: AppDirClient,
{
    fn new() -> Result<Self> {
        let cursor = Term::cursor_position()?;
        let term = Term::start(false)?;
        let size = term.size()?;
        let height = Self::height(size.height);
        let cursor_y_start =
            Self::calculate_and_prepare_cursor_y_start(size.height, cursor.1, height)?;

        Ok(Self {
            term,
            layout: Layout::new(cursor_y_start, size.width, height),
            cursor_start: (cursor.0, cursor_y_start),
        })
    }

    fn resize(&mut self, w: u16, h: u16) {
        let height = Self::height(h);
        self.layout = Layout::new(self.cursor_start.1, w, height)
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
        self.term
            .draw(|f| Clear.render(self.layout.root, f.buffer_mut()))?;
        Term::move_cursor_vertically_to(self.cursor_start.1)?;
        Ok(())
    }
}
