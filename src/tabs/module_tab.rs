use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::DataStore;
use crate::g2_crash_metrics::G2CrashMetrics;
use crate::tabs::tab::Tab;

pub struct ModuleTab {
    pub title: String,
}

impl ModuleTab {
    pub fn new() -> ModuleTab {
        ModuleTab {
            title: "Modules".to_string(),
        }
    }
}

impl Tab for ModuleTab {
    fn on_load(&mut self, app_data: &mut DataStore) {
        app_data.modules.next();
    }

    fn on_key(&mut self, app_data: &mut DataStore, key: KeyCode) {
        match key {
            KeyCode::Up => app_data.modules.previous(),
            KeyCode::Down => app_data.modules.next(),
            _ => {}
        }
    }

    fn get_title(&self) -> &String {
        &self.title
    }

    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, app_data: &mut DataStore, metrics: &mut G2CrashMetrics, area: Rect)
    {
        draw_info(f, metrics, app_data, area);
    }
}

fn draw_info(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, app_state: &mut DataStore, area: Rect)
{
    let chunks = Layout::default().direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ].as_ref(),
        )
        .split(area);

    let modules = metrics.modules.to_string();

    let module_parts: Vec<&str> = modules.split(';').collect();
    let mut module_names: Vec<String> = vec![];
    for module in module_parts.chunks(5) {
        module_names.push(module[0].to_string());
    }
    app_state.modules.items = module_names;

    // Draw modules
    let modules: Vec<ListItem> = app_state.modules
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
        .collect();
    let modules = List::new(modules)
        .block(Block::default().borders(Borders::ALL).title("modules"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");


    f.render_stateful_widget(modules, chunks[0], &mut app_state.modules.state);


    if let Some(selected) = app_state.modules.state.selected() {
        for (i, module) in module_parts.chunks(5).enumerate() {
            if i == selected {
                let mut text = Text::raw("\n");
                text.extend(Text::raw("Module load address: "));
                text.extend(Text::from(Spans::from(vec![
                    Span::raw("0x"),
                    Span::styled(module[1], Style::default().add_modifier(Modifier::BOLD)),
                ])));
                text.extend(Text::raw("\n"));
                text.extend(Text::raw("Module size: "));
                text.extend(Text::from(Spans::from(vec![
                    Span::raw("0x"),
                    Span::styled(module[2], Style::default().add_modifier(Modifier::BOLD)),
                ])));
                text.extend(Text::raw("\n"));
                text.extend(Text::raw("Pdb Guid: "));
                text.extend(Text::from(Spans::from(vec![
                    Span::styled(module[3], Style::default().add_modifier(Modifier::BOLD)),
                ])));
                text.extend(Text::raw("\n"));
                text.extend(Text::raw("Pdb age: "));
                text.extend(Text::from(Spans::from(vec![
                    Span::raw("0x"),
                    Span::styled(module[4], Style::default().add_modifier(Modifier::BOLD)),
                ])));

                let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("statistics"));
                f.render_widget(paragraph, chunks[1]);
            }
        }
    }
}