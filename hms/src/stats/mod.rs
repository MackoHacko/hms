use crate::term::Term;
use anyhow::Result;
use hms_common::app_dir_client::AppDirClient;
use hms_db::manager::HmsDbManager;
use ratatui::{
    layout::{Direction, Rect},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
};
use std::cmp::min;

#[derive(Debug)]
pub struct Stats;

impl Stats {
    pub fn access_count_top_list<'a, P>(db_manager: &'a HmsDbManager<'a, P>, top: i64) -> Result<()>
    where
        P: AppDirClient,
    {
        let top_list = db_manager.with_db(|db| db.fetch_top_snips_by_access(top))?;
        if top_list.is_empty() {
            println!("No snip seems to have been accessed ever... 😔");
            return Ok(());
        }
        let data: Vec<(&str, u64)> = top_list
            .iter()
            .map(|obj| (&obj.alias[..], obj.access_count as u64))
            .collect();

        let barchart = BarChart::default()
            .block(
                Block::default()
                    .title("Alias - Access count")
                    .borders(Borders::ALL)
                    .title_style(Style::default().fg(Color::Cyan))
                    .border_style(Style::default().fg(Color::LightGreen)),
            )
            .bar_width(1)
            .group_gap(0)
            .bar_gap(1)
            .direction(Direction::Horizontal)
            .data(&data)
            .bar_style(Style::default().fg(Color::Green))
            .value_style(Style::default().fg(Color::White).bg(Color::Black));

        let mut term = Term::start(false)?;
        let term_area = term.size()?;

        let num_items = data.len() as u16;
        let min_height_per_bar = 3;
        let chart_height = num_items * min_height_per_bar;
        let adjusted_chart_height = min(chart_height, term_area.height) - (num_items - 1);

        let cursor = term.get_cursor()?;
        let available_height = term_area.height.saturating_sub(cursor.1);
        let render_y = if available_height < adjusted_chart_height {
            let scroll = adjusted_chart_height - available_height;
            Term::scroll_up(scroll)?;
            cursor.1.saturating_sub(scroll)
        } else {
            cursor.1
        };

        let adjusted_chart_width = (term_area.width as f32 * 0.4) as u16;

        let area = Rect::new(0, render_y, adjusted_chart_width, adjusted_chart_height);

        term.draw(|f| f.render_widget(barchart, area))?;

        Term::stop()?;
        println!(); // Move cursor to next line to avoid no trailing new line
        Ok(())
    }
}
