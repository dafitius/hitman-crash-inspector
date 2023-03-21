use crate::tabs::tab::Tab;

pub struct TabsState {
    pub tabs: Vec<Box<dyn Tab>>,
    pub index: usize,
}

#[allow(dead_code)]
impl TabsState {
    pub fn new(tabs: Vec<Box<dyn Tab>>) -> TabsState {
        TabsState { tabs, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tabs.len() - 1;
        }
    }

    pub fn titles(&self) -> Vec<&str>{
        self.tabs.iter().map(|tab| tab.get_title().as_str()).collect()
    }

    pub fn current(&mut self) -> Option<&mut Box<dyn Tab>> {
        self.tabs.get_mut(self.index)
    }
}