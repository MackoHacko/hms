use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

pub const LOGO: &str = "###
  ##      ##  ##    #####
  #####   #######  ##
  ##  ##  ## # ##   #####
  ##  ##  ##   ##       ##
 ###  ##  ##   ##  ######
";

pub struct GradientText {
    text_lines: Vec<String>,
    start: RgbColor,
    end: RgbColor,
}

pub type RgbColor = (u8, u8, u8);

impl GradientText {
    pub fn new(text: &str, start: RgbColor, end: RgbColor) -> Self {
        GradientText {
            text_lines: text.lines().map(String::from).collect(),
            start,
            end,
        }
    }

    fn gradient_color(&self, line_index: usize) -> Color {
        let total_lines = self.text_lines.len();

        let ratio = line_index as f32 / total_lines as f32;
        let mix = |start, end| (start as f32 * (1.0 - ratio) + end as f32 * ratio) as u8;

        Color::Rgb(
            mix(self.start.0, self.end.0),
            mix(self.start.1, self.end.1),
            mix(self.start.2, self.end.2),
        )
    }
}

impl Widget for GradientText {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let code_block = Block::default();
        let inner_area = code_block.inner(area);
        code_block.render(area, buf);

        for (i, line) in self.text_lines.iter().enumerate() {
            if i as u16 >= inner_area.height {
                break;
            }
            let color = Self::gradient_color(&self, i);
            let style = Style::default().fg(color);

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
