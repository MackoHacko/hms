use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug)]
pub struct GuiLayout {
    pub logo_area: Rect,
    pub alias_area: Rect,
    pub snip_area: Rect,
    pub search_area: Rect,
}

impl GuiLayout {
    pub fn new(term_area: Rect) -> Self {
        let root_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(90), // main
                Constraint::Percentage(5),  // search
                Constraint::Percentage(5),  // tools
            ])
            .split(Rect::new(
                5,
                1,
                term_area.width.saturating_sub(5),
                term_area.height.saturating_sub(1),
            ));

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35), // left
                Constraint::Percentage(65), // right
            ])
            .split(root_layout[0]);

        let main_left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20), // logo
                Constraint::Percentage(5),  // whitespace
                Constraint::Percentage(75), // alias list
            ])
            .split(main_layout[0]);

        let main_right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100), // snip display
            ])
            .split(main_layout[1]);

        let logo = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(main_left[0]);

        let alias_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(main_left[2]);

        let search_bar_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(root_layout[1]);

        let snip_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(main_right[0]);

        Self {
            logo_area: logo[0],
            alias_area: alias_layout[0],
            snip_area: snip_layout[0],
            search_area: search_bar_layout[0],
        }
    }
}
