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

pub struct SettingsTab {
    pub title: String,
}

impl SettingsTab {
    pub fn new() -> SettingsTab {
        SettingsTab {
            title: "Settings".to_string(),
        }
    }
}


impl Tab for SettingsTab {
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
        draw_settings(f, metrics, chunks[0]);
    }
}

fn draw_settings(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, area: Rect)
{
    let chunks = Layout::default().direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(45),
                Constraint::Percentage(45),
                Constraint::Percentage(5),
            ].as_ref(),
        )
        .split(area);


    let settings = metrics.settings_info.to_string();

    let binding = settings.replace("Graphics Settings:", "");
    let lines = binding.split('\n').collect::<Vec<&str>>();

    let (left_lines, right_lines) = lines.split_at(lines.len() / 2);

    let mut left = Text::raw("\n\n");
    for line in left_lines {
        left.extend(Text::raw(*line));
    }


    let mut right = Text::raw("\n\n");
    for line in right_lines {
        right.extend(Text::raw(*line));
    }


    let paragraph = Paragraph::new(left);
    f.render_widget(paragraph, chunks[1]);

    let paragraph = Paragraph::new(right);
    f.render_widget(paragraph, chunks[2]);


    let block = Block::default().borders(Borders::ALL).title("Graphics Settings");
    f.render_widget(block, area);
}