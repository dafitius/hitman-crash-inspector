extern crate core;

mod app;
mod crossterm;
mod ui;
mod tabs;
mod controls;
mod g2_crash_metrics;
mod nt_status_enum;

use crate::crossterm::run;
use argh::FromArgs;
use std::{error::Error, time::Duration};

#[derive(Debug, FromArgs)]
#[argh(description = "Hitman crash inspector")]
struct Cli {

    #[argh(option, default = "150", description="time in ms between two ticks (default: 150)")]
    tick_rate: u64,
    #[argh(option, default = "true", description="whether unicode symbols are used to improve the overall look of the app (default: true)")]
    enhanced_graphics: bool,
    //#[argh(option, description="path to the metrics file")]
    #[argh(option, description="path to the metrics file")]
    metrics_path: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics, cli.metrics_path)?;
    Ok(())
}
