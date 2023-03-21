use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::backend::{CrosstermBackend};
use tui::Frame;
use tui::layout::Rect;
use crate::app::DataStore;
use crate::g2_crash_metrics::G2CrashMetrics;

pub trait Tab {
    fn on_load(&mut self, app_data: &mut DataStore);
    fn on_key(&mut self, app_data: &mut DataStore, key: KeyCode);
    fn get_title(&self) -> &String;
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, app_data: &mut DataStore, metrics: &mut G2CrashMetrics, area: Rect);
}