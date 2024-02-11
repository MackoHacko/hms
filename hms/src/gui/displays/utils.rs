use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::rc::Rc;

pub fn create_layout(area: Rect, direction: Direction, split_percentages: Vec<u16>) -> Rc<[Rect]> {
    Layout::default()
        .direction(direction)
        .constraints(
            split_percentages
                .into_iter()
                .map(Constraint::Percentage)
                .collect::<Vec<_>>(),
        )
        .split(area)
}
