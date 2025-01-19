use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Debug, Default)]
pub struct SnipValue {
    snip: String,
}

impl SnipValue {
    pub fn new(snip: String) -> Self {
        Self { snip }
    }
}

impl Widget for SnipValue {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let code_block = Block::default()
            .borders(Borders::ALL)
            .title_bottom(Span::raw("Delete: ctrl+d"))
            .title_alignment(Alignment::Right)
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
