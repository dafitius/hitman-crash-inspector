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

pub struct VrTab {
    pub title: String,
}

impl VrTab {
    pub fn new() -> VrTab {
        VrTab {
            title: "VR".to_string(),
        }
    }
}

impl Tab for VrTab {
    fn on_load(&mut self, _app_data: &mut DataStore) {}

    fn on_key(&mut self, _app_data: &mut DataStore, _key: KeyCode) {}

    fn get_title(&self) -> &String {
        &self.title
    }

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, _app_data: &mut DataStore, metrics: &mut G2CrashMetrics, area: Rect)
    {
        draw_info(f, metrics, area);
    }
}

fn draw_info(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, area: Rect)
{
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref(),
        )
        .split(area);


    let vr_data = metrics.vr_data.to_string();

    let mut text = Text::raw("\n");
    text.extend(Text::raw(vr_data));


    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("VR data"));
    f.render_widget(paragraph, chunks[0]);


    let vr_desc = metrics.vr_hdm_description.to_string();

    let mut text = Text::raw("\n");
    if !vr_desc.is_empty() {
        text.extend(Text::raw(vr_desc));
    } else {
        text.extend(Text::raw("No description found!"));
    }

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("VR header description"));
    f.render_widget(paragraph, chunks[1]);
}