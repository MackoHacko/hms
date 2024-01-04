use hms_db::models::Snip;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

#[derive(Debug, Default)]
pub struct SnipListState {
    offset: usize,
    selected: usize,
    snips: Vec<Snip>,
}

impl SnipListState {
    pub fn selected_snip_index(&self) -> usize {
        self.selected
    }

    pub fn selected_snip(&self) -> Option<&Snip> {
        self.snips.get(self.selected_snip_index())
    }

    pub fn select(&mut self, index: usize) {
        if index <= self.snips.len().saturating_sub(1) {
            self.selected = index
        }
    }

    pub fn set_snips(&mut self, snips: Vec<Snip>) {
        self.snips = snips;
        if self.snips.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.snips.len() {
            self.selected = self.snips.len().saturating_sub(1);
        }
    }

    pub fn selected_snip_value(&mut self) -> Option<String> {
        self.selected_snip().map(|f| f.value.to_owned())
    }
}

struct DrawState<'a> {
    buf: &'a mut Buffer,
    list_area: Rect,
    x: u16,
    y: u16,
    state: &'a SnipListState,
}

impl DrawState<'_> {
    fn draw(&mut self, s: &str, style: Style) {
        let cx = self.list_area.left() + self.x;
        let cy = self.list_area.top() + self.y;
        let w = (self.list_area.width - self.x) as usize;
        self.x += self.buf.set_stringn(cx, cy, s, w, style).0 - cx;
    }

    fn render_snip(&mut self, s: &Snip) {
        let is_selected = self.y as usize + self.state.offset == self.state.selected;
        let default_style = Style::default();
        let selected_style = default_style.fg(Color::White).add_modifier(Modifier::BOLD);

        if is_selected {
            let fill_style = Style::default().bg(Color::Indexed(233));
            let fill_str = " ".repeat(self.list_area.width.saturating_sub(1) as usize);
            self.draw(&fill_str, fill_style);
            self.x = 0; // Reset x to start of line
        }

        let (prefix, style) = if is_selected {
            (">", selected_style)
        } else {
            (" ", default_style)
        };

        self.draw(prefix, style);
        for section in s.alias.split_ascii_whitespace() {
            self.x += 1;
            if self.x > self.list_area.width {
                return;
            }
            self.draw(section, style);
        }
    }
}

pub struct SnipList;

impl SnipList {
    fn get_items_bounds(&self, snips: &Vec<Snip>, offset: usize, height: usize) -> (usize, usize) {
        let total_items = snips.len();
        let start = offset;
        let end = usize::min(start + height, total_items);
        (start, end)
    }
}

impl StatefulWidget for SnipList {
    type State = SnipListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::LEFT | Borders::BOTTOM)
            .border_style(Style::new().dark_gray().dim())
            .border_type(ratatui::widgets::BorderType::Rounded);

        let list_area = block.inner(area);
        block.render(area, buf);

        if list_area.width < 1 || area.height < 1 || state.snips.is_empty() {
            return;
        }

        let area_height = list_area.height as usize;
        let (start, end) = self.get_items_bounds(&state.snips, state.offset, area_height);

        if state.selected >= end {
            state.offset += state.selected - end + 1;
        } else if state.selected < start {
            state.offset -= start - state.selected;
        }

        let mut ds = DrawState {
            buf,
            list_area,
            x: 0,
            y: 0,
            state,
        };

        for snip in state
            .snips
            .iter()
            .skip(state.offset)
            .take(end.saturating_sub(start))
        {
            ds.render_snip(snip);

            // reset line
            ds.y += 1;
            ds.x = 0;
        }
    }
}
