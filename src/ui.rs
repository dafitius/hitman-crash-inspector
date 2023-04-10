use std::io::Stdout;
use tui::backend::{CrosstermBackend};
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Paragraph, Tabs};
use crate::app::App;


pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles()
        .iter()
        .enumerate()
        .map(|(i,t)| Spans::from(Span::styled(format!("{}: {}", i+1, *t), Style::default().fg(Color::LightRed))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    if let Some(tab) = app.tabs.current() {
        tab.draw(f, &mut app.data, &mut app.metrics, chunks[1]);
        let align = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(1),
                Constraint::Percentage(99)
            ].as_ref())
            .split(chunks[0]);

        let align2 = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(60)
        ].as_ref())
        .split(align[1]);

        let live_update = app.data.should_live_update.to_string();
        let controls = vec![
            Span::styled(" 1-8 ", Style::default().add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::styled("switch tab", Style::default()),
            Span::raw(" ――― "),

            Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::styled("ave or ", Style::default()),
            Span::styled("i", Style::default().add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::styled("mport file", Style::default()),
            Span::raw(" ――― "),

            Span::styled("Toggle ", Style::default()),
            Span::styled("l", Style::default().add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
            Span::styled("ive refresh (", Style::default()),
            Span::styled(&live_update, Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(") ", Style::default()),
        ];

        f.render_widget( Paragraph::new(Text::from(Spans::from(controls))), align2[1]);
    }



}