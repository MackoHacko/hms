use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

pub struct SnipDisplay {
    snip: String,
}

impl SnipDisplay {
    pub fn new(snip: String) -> Self {
        Self { snip }
    }
}

impl Widget for SnipDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let code_block = Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Rounded);
        let inner_area = code_block.inner(area);
        code_block.render(area, buf);

        let style = Style::default().fg(Color::White);
        let lines = self.snip.lines();

        for (i, line) in lines.enumerate() {
            if i as u16 >= inner_area.height {
                break; // Stop rendering if we run out of vertical space
            }
            buf.set_stringn(
                inner_area.x,
                inner_area.y + i as u16,
                line,
                inner_area.width as usize,
                style,
            );
        }
    }
}
