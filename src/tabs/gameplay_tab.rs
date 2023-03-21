use std::io::Stdout;
use cgmath::{Deg, Euler, Matrix3, Quaternion, Rad};
use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use crate::app::DataStore;
use crate::g2_crash_metrics::G2CrashMetrics;
use crate::tabs::tab::Tab;

pub struct GameplayTab {
    pub title: String,
}

impl GameplayTab {
    pub fn new() -> GameplayTab {
        GameplayTab {
            title: "Gameplay".to_string(),
        }
    }
}


impl Tab for GameplayTab {
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
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ].as_ref(),
            )
            .split(area);
        draw_info(f, metrics, chunks[0]);

        let chunks_right = Layout::default().direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref(),
            )
            .split(chunks[1]);

        draw_camera(f, metrics, chunks_right[0]);
        draw_online(f, metrics, chunks_right[1]);
    }
}

fn draw_online(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, area: Rect) {
    let net_role = metrics.net_role.to_string();
    let net_role_tag = vec![
        Span::raw("Role: "),
        Span::styled(&net_role, Style::default().add_modifier(Modifier::BOLD)),
    ];

    let server_version = metrics.online_server_version.to_string();
    let server_version_tag = vec![
        Span::raw("server version: "),
        Span::styled(&server_version, Style::default().add_modifier(Modifier::BOLD)),
    ];


    let mut text = Text::raw("\n");
    text.extend(Text::from(Spans::from(net_role_tag)));
    text.extend(Text::raw("\n"));
    text.extend(Text::from(Spans::from(server_version_tag)));
    text.extend(Text::raw("\n"));

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("online"));
    f.render_widget(paragraph, area);
}

fn draw_camera(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, area: Rect)
{
    let camera_str = metrics.camera.to_string();
    let test = camera_str.split(',');
    let camera: Vec<f32> = test.map(|num| num.trim().parse::<f32>().unwrap_or(0.0)).collect();

    if camera.len() != 12 {
        return;
    }

    let rot_mat: Matrix3<f32> = Matrix3::new(
        camera[0], camera[1], camera[2],
        camera[3], camera[4], camera[5],
        camera[6], camera[7], camera[8]);

    let quaternion: Quaternion<f32> = Quaternion::from(rot_mat);

    let euler: Euler<Rad<f32>> = Euler::from(quaternion);
    let rot_x = Deg::from(-euler.x);
    let rot_y = Deg::from(-euler.y);
    let rot_z = Deg::from(-euler.z);

    let table = Table::new(vec![
        Row::new(vec![
            Cell::from("Rotation").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from(rot_x.0.to_string()),
            Cell::from(rot_y.0.to_string()),
            Cell::from(rot_z.0.to_string()),
        ]).height(2),
        Row::new(vec![Cell::from("")]),
        Row::new(vec![
            Cell::from("Position").style(Style::default().add_modifier(Modifier::BOLD)),
            Cell::from(camera.get(9).unwrap().to_string()),
            Cell::from(camera.get(10).unwrap().to_string()),
            Cell::from(camera.get(11).unwrap().to_string()),
        ]).height(2),
    ])
// You can set the style of the entire Table.
        .style(Style::default().fg(Color::White))
// It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["", "x", "y", "z"])
                .style(Style::default().add_modifier(Modifier::BOLD))
                // If you want some space between the header and the rest of the rows, you can always
                // specify some margin at the bottom.
                .bottom_margin(1)
        )
// As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Camera").borders(Borders::ALL))
// Columns widths are constrained in the same way as Layout...
        .widths(&[Constraint::Length(10), Constraint::Length(10), Constraint::Length(10), Constraint::Length(10)])
// ...and they can be separated by a fixed spacing.
        .column_spacing(1);

    f.render_widget(table, area);
}

fn draw_info(f: &mut Frame<CrosstermBackend<Stdout>>, metrics: &mut G2CrashMetrics, area: Rect)
{
    let scene = metrics.scene.to_string();
    let scene_tag = vec![
        Span::raw("Scene: "),
        Span::styled(&scene, Style::default().add_modifier(Modifier::BOLD)),
    ];

    let uptimems = metrics.uptimems.to_string();
    let uptime_tag = vec![
        Span::raw("uptime: "),
        Span::styled(&uptimems, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" ms"),
    ];


    let mut text = Text::raw("\n");
    text.extend(Text::from(Spans::from(scene_tag)));
    text.extend(Text::raw("\n"));
    text.extend(Text::from(Spans::from(uptime_tag)));
    text.extend(Text::raw("\n"));

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("state"));
    f.render_widget(paragraph, area);
}