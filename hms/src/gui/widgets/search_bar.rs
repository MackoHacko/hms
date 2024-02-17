use ratatui::widgets::{Block, Paragraph};

pub struct SearchBar;

impl SearchBar {
    pub fn new(query: String) -> Paragraph<'static> {
        Paragraph::new(format!("Search: {}_", query)).block(Block::default())
    }
}
