use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Text;
use tui::widgets::{Block, Borders, Paragraph};
use crate::app::DataStore;
use crate::g2_crash_metrics::{G2CrashMetrics, G2Exception};
use crate::nt_status_enum::NTSTATUS;

use crate::tabs::tab::Tab;

pub struct ExceptionTab {
    pub title: String,
}

impl ExceptionTab {
    pub fn new() -> ExceptionTab {
        ExceptionTab {
            title: "Exception".to_string(),
        }
    }
}

impl Tab for ExceptionTab {
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
    let modules = metrics.modules.to_string();
    let modules: Vec<String> = modules.split(';').map(|x| x.to_string()).collect();

    let exception: G2Exception = metrics.exception;

    let mut text = Text::raw("\n");
    text.extend(Text::raw(format!(" flag: 0x{:x}", exception.exception_flags)));
    text.extend(Text::raw("\n"));
    if let Some(nt_status) = NTSTATUS.get(&exception.exception_code) {
        text.extend(Text::raw(format!(" code: 0x{:x} ({})", exception.exception_code, nt_status)));
    } else {
        text.extend(Text::raw(format!(" code: 0x{:x}", exception.exception_code)));
    }


    text.extend(Text::raw("\n"));

    let address = exception.exception_address;
    let mut dist = u64::MAX;
    let mut module_name = "unknown";
    for module in modules.chunks(5) {
        if let Ok(module_address) = u64::from_str_radix(module[1].as_str(), 16) {
            if let Some(diff) = address.checked_sub(module_address) {
                if diff < dist {
                    dist = diff;
                    module_name = module[0].as_str();
                }
            }
        }
    }
    text.extend(Text::raw(format!(" address: 0x{address:x}  (in {module_name})")));
    text.extend(Text::raw("\n"));

    text.extend(Text::raw(format!(" num parameters: {}", exception.exception_num_parameters)));
    text.extend(Text::raw("\n"));

    text.extend(Text::raw(" Exception information: "));
    text.extend(Text::raw(format!("  1. 0x{:x}", exception.exception_information_01)));
    text.extend(Text::raw(format!("  2. 0x{:x}", exception.exception_information_02)));
    text.extend(Text::raw(format!("  3. 0x{:x}", exception.exception_information_03)));
    text.extend(Text::raw("\n"));


    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Exception"));
    f.render_widget(paragraph, area);
}