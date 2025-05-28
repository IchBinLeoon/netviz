use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Padding, Sparkline, SparklineBar, Widget};

use crate::define_directional_history;
use crate::history::{DEFAULT_HISTORY_SIZE, Direction, History};

define_directional_history!(PacketHistory, u64, DEFAULT_HISTORY_SIZE, {}, {
    pub fn add_bar(&mut self, value: u64) {
        self.history.add(value);
    }
});

impl Widget for &PacketHistory {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut data = Vec::new();

        let (title, color) = match self.direction {
            Direction::Download => ("Incoming Packets", Color::Cyan),
            Direction::Upload => ("Outgoing Packets", Color::Magenta),
        };

        let block = Block::bordered()
            .title(Span::styled(title, color))
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));

        let block_area = block.inner(area);

        block.render(area, buf);

        for value in self.history.values().iter().rev() {
            for _ in 0..block_area
                .width
                .div_ceil(self.history.max_size as u16)
                .max(1)
            {
                data.push(SparklineBar::from(value))
            }
        }

        let sparkline = Sparkline::default().data(data).style(color);

        sparkline.render(block_area, buf);
    }
}
