extern crate core;

mod app;
mod crossterm;
mod ui;
mod tabs;
mod controls;
mod g2_crash_metrics;
mod utils;
mod nt_status_enum;

use crate::crossterm::run;
use argh::FromArgs;
use std::{error::Error, time::Duration};

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "150")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics)?;
    Ok(())
}
