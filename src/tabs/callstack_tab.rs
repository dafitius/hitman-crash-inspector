use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};
use crate::app::DataStore;
use crate::g2_crash_metrics::G2CrashMetrics;
use crate::tabs::tab::Tab;

pub struct CallstackTab {
    pub title: String,
}

impl CallstackTab {
    pub fn new() -> CallstackTab {
        CallstackTab {
            title: "Callstack".to_string(),
        }
    }
}

impl Tab for CallstackTab {
    fn on_load(&mut self, app_data: &mut DataStore) {
        app_data.callstack.next();
    }

    fn on_key(&mut self, app_data: &mut DataStore, key: KeyCode) {
        match key {
            KeyCode::Up => app_data.callstack.previous(),
            KeyCode::Down => app_data.callstack.next(),
            _ => {}
        }
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, app_data: &mut DataStore, metrics: &mut G2CrashMetrics, area: Rect)
    {
        let chunks = Layout::default().direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(100),
                ].as_ref(),
            )
            .split(area);
        draw_info(f, metrics, app_data, chunks[0]);
    }
}

fn draw_info(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, app_state: &mut DataStore, area: Rect)
{
    let callstack = metrics.callstack.to_string();
    let mut callstack: Vec<String> = callstack.split(';').map(|x| x.to_string()).collect();

    let modules = metrics.modules.to_string();
    let modules: Vec<String> = modules.split(';').map(|x| x.to_string()).collect();


    for call in callstack.iter_mut() {
        if let Ok(address) = u64::from_str_radix(call, 16) {
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
            *call = format!("0x{}  (in {})", *call, module_name);
        }
    }

    app_state.callstack.items = callstack;

    let items: Vec<ListItem> = app_state.callstack
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
        .collect();
    let callstack_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Callstack"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("");


    f.render_stateful_widget(callstack_list, area, &mut app_state.callstack.state);
}