use std::collections::HashMap;
use std::io::{Error, Stdout, stdout};
use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Padding, Paragraph};
use ratatui::{Frame, Terminal};
use sysinfo::Networks;

use crate::history::packet::PacketHistory;
use crate::history::traffic::TrafficHistory;

pub struct App {
    networks: Networks,

    rx_traffic: HashMap<String, TrafficHistory>,
    tx_traffic: HashMap<String, TrafficHistory>,

    rx_packets: HashMap<String, PacketHistory>,
    tx_packets: HashMap<String, PacketHistory>,

    index: usize,
    second: u64,

    is_paused: bool,
    should_exit: bool,
}

impl App {
    const TICK_RATE: Duration = Duration::from_secs(1);

    pub fn init() -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            rx_traffic: HashMap::new(),
            tx_traffic: HashMap::new(),
            rx_packets: HashMap::new(),
            tx_packets: HashMap::new(),
            index: 0,
            second: 0,
            is_paused: false,
            should_exit: false,
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let mut stdout = stdout();

        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run(&mut terminal);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        res
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {
        self.tick();
        let mut last_tick = Instant::now();

        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = Self::TICK_RATE.saturating_sub(last_tick.elapsed());
            self.handle_events(timeout)?;

            if last_tick.elapsed() >= Self::TICK_RATE {
                self.tick();
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn tick(&mut self) {
        self.networks.refresh(true);

        if self.is_paused {
            return;
        }

        for (name, network) in self.networks.iter() {
            let rx_traffic = network.received();
            let tx_traffic = network.transmitted();

            self.rx_traffic
                .entry(name.to_owned())
                .or_insert(TrafficHistory::download())
                .add_point((self.second as f64, rx_traffic as f64));

            self.tx_traffic
                .entry(name.to_owned())
                .or_insert(TrafficHistory::upload())
                .add_point((self.second as f64, tx_traffic as f64));

            let rx_packets = network.packets_received();
            let tx_packets = network.packets_transmitted();

            self.rx_packets
                .entry(name.to_owned())
                .or_insert(PacketHistory::download())
                .add_bar(rx_packets);

            self.tx_packets
                .entry(name.to_owned())
                .or_insert(PacketHistory::upload())
                .add_bar(tx_packets);
        }

        self.second += 1;
    }

    fn handle_events(&mut self, timeout: Duration) -> Result<(), Error> {
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        self.should_exit = true;
                    }
                    KeyCode::Char('p') => {
                        self.is_paused = !self.is_paused;
                    }
                    KeyCode::Left => self.previous(),
                    KeyCode::Right => self.next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let (name, data) = self.networks.iter().nth(self.index).unwrap();

        let root_chunks =
            Layout::vertical([Constraint::Length(5), Constraint::Min(0)]).split(frame.area());

        let mut line = vec![Span::from(name).green()];

        let mut ips = data.ip_networks().to_vec();
        ips.sort();

        for ip in ips {
            line.extend([
                Span::from(" [").dark_gray(),
                Span::from(ip.addr.to_string()).gray(),
                Span::from("]").dark_gray(),
            ]);
        }

        let interface = Paragraph::new(Line::from(line)).block(
            Block::bordered()
                .title(format!(
                    "Interface ({}/{}) (←/→) - [p] Pause - [q] Quit",
                    self.index + 1,
                    self.networks.len()
                ))
                .border_type(BorderType::Rounded)
                .padding(Padding::proportional(1)),
        );

        frame.render_widget(interface, root_chunks[0]);

        let content_chunks =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(root_chunks[1]);

        let left_chunks =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(content_chunks[0]);

        frame.render_widget(self.rx_traffic.get(name).unwrap(), left_chunks[0]);
        frame.render_widget(self.tx_traffic.get(name).unwrap(), left_chunks[1]);

        let right_chunks =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(content_chunks[1]);

        frame.render_widget(self.rx_packets.get(name).unwrap(), right_chunks[0]);
        frame.render_widget(self.tx_packets.get(name).unwrap(), right_chunks[1]);
    }

    fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn next(&mut self) {
        if self.index + 1 < self.networks.len() {
            self.index += 1;
        }
    }
}
