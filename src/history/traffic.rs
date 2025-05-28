use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{
    Axis, Block, BorderType, Chart, Dataset, GraphType, Padding, Paragraph, Widget,
};

use crate::define_directional_history;
use crate::history::{DEFAULT_HISTORY_SIZE, Direction, History};
use crate::utils::format_bytes;

define_directional_history!(
    TrafficHistory,
    (f64, f64),
    DEFAULT_HISTORY_SIZE,
    {
        min_value: f64 = f64::MAX,
        max_value: f64 = f64::MIN,
        total: f64 = 0.0,
        samples: u64 = 0,
    },
    {
        pub fn add_point(&mut self, value: (f64, f64)) {
            self.history.add(value);

            let (_, y) = value;

            if y < self.min_value {
                self.min_value = y;
            }

            if y > self.max_value {
                self.max_value = y;
            }

            self.total += y;
            self.samples += 1;
        }

        fn average_value(&self) -> f64 {
            self.total / self.samples as f64
        }
    }
);

impl Widget for &TrafficHistory {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let data: Vec<(f64, f64)> = self
            .history
            .values()
            .iter()
            .map(|(x, y)| (*x, *y))
            .collect();

        let x_bounds = [data[0].0, data[data.len() - 1].0];
        let y_bounds = [0.0, data.iter().map(|(_, y)| *y).fold(f64::MIN, f64::max)];

        let (title, color) = match self.direction {
            Direction::Download => ("Download ↓", Color::Cyan),
            Direction::Upload => ("Upload ↑", Color::Magenta),
        };

        let block = Block::bordered()
            .title(Span::styled(title, color))
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));

        let block_area = block.inner(area);

        block.render(area, buf);

        let block_chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(block_area);

        let values = [
            format!(
                "Current: {:>11}",
                format_bytes(data[data.len() - 1].1, true)
            ),
            format!("Average: {:>11}", format_bytes(self.average_value(), true)),
            format!("Min: {:>11}", format_bytes(self.min_value, true)),
            format!("Max: {:>11}", format_bytes(self.max_value, true)),
            format!("Total: {:>11}", format_bytes(self.total, false)),
        ];

        let value_chunks =
            Layout::horizontal([Constraint::Percentage(20); 5]).split(block_chunks[0]);

        for (i, value) in values.iter().enumerate() {
            let paragraph = Paragraph::new(Span::from(value)).centered();

            paragraph.render(value_chunks[i], buf);
        }

        let datasets = vec![
            Dataset::default()
                .data(&data)
                .marker(Marker::Dot)
                .graph_type(GraphType::Line)
                .style(color),
        ];

        let chart = Chart::new(datasets)
            .x_axis(
                Axis::default()
                    .labels(vec![
                        Span::from(format!("{}s", x_bounds[0])),
                        Span::from(format!("{:.1}s", (x_bounds[0] + x_bounds[1]) / 2.0)),
                        Span::from(format!("{}s", x_bounds[1])),
                    ])
                    .bounds(x_bounds),
            )
            .y_axis(
                Axis::default()
                    .labels(vec![
                        Span::from(format!("{:>11}", format_bytes(y_bounds[0], true))),
                        Span::from(format!(
                            "{:>11}",
                            format_bytes((y_bounds[0] + y_bounds[1]) / 2.0, true)
                        )),
                        Span::from(format!("{:>11}", format_bytes(y_bounds[1], true))),
                    ])
                    .bounds(y_bounds),
            );

        chart.render(block_chunks[1], buf);
    }
}
