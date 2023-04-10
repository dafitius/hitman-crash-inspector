use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use crossterm::event::KeyCode;
use native_dialog::FileDialog;
use notify::{PollWatcher, RecursiveMode, Watcher, Config, Event};
use crate::controls::stateful_list::StatefulList;
use crate::controls::stateful_tabs::TabsState;
use crate::g2_crash_metrics::G2CrashMetrics;
use crate::tabs::callstack_tab::CallstackTab;
use crate::tabs::exception_tab::ExceptionTab;
use crate::tabs::gameplay_tab::GameplayTab;
use crate::tabs::gpu_tab::GpuTab;
use crate::tabs::module_tab::ModuleTab;
use crate::tabs::settings_tab::SettingsTab;
use crate::tabs::system_tab::SystemTab;
use crate::tabs::vr_tab::VrTab;

pub struct DataStore {
    pub should_quit: bool,
    pub enhanced_graphics: bool,
    pub should_live_update: bool,

    //modules tab storage
    pub modules: StatefulList<String>,

    //callstack tab storage
    pub callstack: StatefulList<String>,

    //watched file props
    pub path: String,
    receiver: Receiver<notify::Result<Event>>,
    watcher: PollWatcher,

}


pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState,
    pub data: DataStore,
    pub metrics: G2CrashMetrics,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_secs(1));
        let (sender, receiver) = channel();
        let watcher = PollWatcher::new(sender, config).unwrap();

        App {
            title,
            tabs: TabsState::new(
                vec![
                    Box::new(GameplayTab::new()),
                    Box::new(SystemTab::new()),
                    Box::new(SettingsTab::new()),
                    Box::new(GpuTab::new()),
                    Box::new(VrTab::new()),
                    Box::new(ModuleTab::new()),
                    Box::new(CallstackTab::new()),
                    Box::new(ExceptionTab::new()),
                ]),
            data: DataStore {
                should_quit: false,
                should_live_update: true,

                modules: StatefulList::with_items(vec![]),
                callstack: StatefulList::with_items(vec![]),
                path: String::new(),
                receiver,
                watcher,

                enhanced_graphics,
            },
            metrics: G2CrashMetrics::default(),
        }
    }

    pub fn on_load(&mut self) {
        if self.data.path.is_empty() {
            self.import_metrics();
        }

        for tab in self.tabs.tabs.iter_mut() {
            tab.on_load(&mut self.data);
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(dig) if dig.is_numeric() => self.tabs.index = u32::clamp(dig.to_digit(0x10).unwrap_or(0), 1, self.tabs.tabs.len() as u32) as usize - 1,
            KeyCode::Char('l') => {
                self.data.should_live_update = !self.data.should_live_update;
                if self.data.should_live_update {
                    let path = self.data.path.clone();
                    self.update_metrics(path.as_str());
                }
            }
            KeyCode::Char('s') => {
                self.save_metrics();
            }
            KeyCode::Char('i') => {
                self.import_metrics();
            }
            _ => if let Some(tab) = self.tabs.current() {
                tab.on_key(&mut self.data, key);
            },
        }
    }

    pub fn on_tick(&mut self) {
        while let Ok(event) = self.data.receiver.recv_timeout(Duration::from_millis(10)) {
            if let Ok(event) = event {
                if let Some(path) = event.paths.last() {
                    if self.data.should_live_update {
                        self.update_metrics(path.to_str().unwrap());
                    }
                }
            }
        }
    }


    fn import_metrics(&mut self) {
        let path = FileDialog::new()
            .set_location("~/AppData")
            .add_filter("crash_metrics", &["dat", "json"])
            .show_open_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => return,
        };

        self.data.watcher.watch(&path, RecursiveMode::Recursive).unwrap();

        if let Some(path_str) = path.to_str() {
            self.data.path = path_str.to_string();
            self.update_metrics(path_str);
        }
    }

    fn update_metrics(&mut self, path: &str) {
        if path.ends_with(".json") {
            if let Ok(json_string) = std::fs::read_to_string(path) {
                match serde_json::from_str::<G2CrashMetrics>(&json_string) {
                    Ok(crash_metrics) => { self.metrics = crash_metrics }
                    Err(err) => panic!("{}", err),
                }
            }
        } else {
            match G2CrashMetrics::new(path) {
                Ok(crash_metrics) => { self.metrics = crash_metrics }
                Err(err) => panic!("{}", err),
            }
        }
    }

    fn save_metrics(&mut self) {
        let path = FileDialog::new()
            .set_location("~/Documents")
            .add_filter("crash_metrics.json", &["json"])
            .show_save_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => return,
        };

        // Convert the metrics to a JSON string.
        let serialized = serde_json::to_string(&self.metrics).unwrap();

        std::fs::write(path, serialized.as_bytes()).expect("Couldnt write to file");
    }
}