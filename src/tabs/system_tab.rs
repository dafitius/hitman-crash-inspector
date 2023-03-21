use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Paragraph};
use crate::app::DataStore;
use crate::g2_crash_metrics::G2CrashMetrics;
use crate::tabs::tab::Tab;

pub struct SystemTab {
    pub title: String,
}

impl SystemTab {
    pub fn new() -> SystemTab {
        SystemTab {
            title: "System".to_string(),
        }
    }
}


impl Tab for SystemTab {
    fn on_load(&mut self, _app_data: &mut DataStore) {}

    fn on_key(&mut self, _app_data: &mut DataStore, _key: KeyCode) {}

    fn get_title(&self) -> &String {
        &self.title
    }

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, _app_data: &mut DataStore, metrics: &mut G2CrashMetrics, area: Rect)
    {
        let chunks = Layout::default().direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(100),
                ].as_ref(),
            )
            .split(area);
        draw_info(f, metrics, chunks[0]);
    }
}

fn draw_info(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, area: Rect)
{
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref(),
        )
        .split(area);

    let info = metrics.system_info.to_string();

    let mut text = Text::raw(info.replace("System Info:", ""));
    text.extend(Text::raw("\n"));

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("System Info"));
    f.render_widget(paragraph, chunks[0]);


    //OS block
    let os = metrics.operating_system.to_string();
    let os_tag = vec![
        Span::styled(&os, Style::default().add_modifier(Modifier::BOLD)),
    ];

    let mut text2 = Text::raw("\n");
    text2.extend(Text::from(Spans::from(os_tag)));

    let paragraph = Paragraph::new(text2).block(Block::default().borders(Borders::ALL).title("Operating system"));
    f.render_widget(paragraph, chunks[1]);
}