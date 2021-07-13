use crate::conf::conf::Conf;

pub struct App {
    pub tabs: TabsState,
}

impl App {
    pub fn new(conf: &Conf) -> App {
        App {
            tabs: TabsState::new(conf.get_todo_types())
        }

    }
}

pub struct TabsState<> {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    pub fn get_current_todo_type(&self) -> Option<&String> {
        self.titles.get(self.index)
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
