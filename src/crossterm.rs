use crate::{app::App, ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use std::io::Stdout;
use anyhow::anyhow;
use crossterm::event::KeyEventKind;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

pub fn run(tick_rate: Duration, enhanced_graphics: bool, metrics_path: Option<String>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("Hitman 3 crash inspector", enhanced_graphics);
    let res = run_app(&mut terminal, app, tick_rate, metrics_path);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
    tick_rate: Duration,
    metrics_path: Option<String>,
) -> Result<(), anyhow::Error> {

    if let Some(path) = metrics_path{
        app.data.path = path.clone();

        match app.update_metrics(path.as_str()){
            Ok(_) => {},
            Err(e) => {
                app.data.should_quit = true;
                app.data.quit_msg = format!("Error: {e}");
            }
        }
    }

    app.on_load();
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('c') => {
                            app.data.should_quit = true;
                        }
                        _ => app.on_key(key.code)
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.data.should_quit {
            if app.data.quit_msg.is_empty(){
                return Ok(());
            }
            else{
                return Err(anyhow!(app.data.quit_msg))
            }
        }
    }
}
