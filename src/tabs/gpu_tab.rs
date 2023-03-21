use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use crate::app::DataStore;
use crate::g2_crash_metrics::G2CrashMetrics;
use crate::tabs::tab::Tab;

pub struct GpuTab {
    pub title: String,
}

impl GpuTab {
    pub fn new() -> GpuTab {
        GpuTab {
            title: "GPU".to_string(),
        }
    }
}

impl Tab for GpuTab {
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
    let gpu_crash = metrics.gpu_crash_report.to_string();

    let mut text = Text::raw("\n");
    if !gpu_crash.is_empty() {
        text.extend(Text::raw(gpu_crash));
    } else {
        text.extend(Text::raw("No crash detected!"));
    }

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Gpu Crash Report"));
    f.render_widget(paragraph, area);
}