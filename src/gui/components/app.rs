use crate::conf::conf::Conf;

pub struct App {
    pub tabs: TabsState,
    pub popup_visible: bool,
}

impl App {
    pub fn new(conf: &Conf) -> App {
        App {
            tabs: TabsState::new(conf.get_todo_types()),
            popup_visible: false,
        }
    }

    // TODO: allow a popup to be passed to the function
    pub fn toggle_popup(&mut self) {
        match self.popup_visible {
            true => self.popup_visible = false,
            false => self.popup_visible = true
        }
    }

}

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    #[allow(dead_code)]
    pub fn get_current_todo_type(&self) -> Option<&String> {
        self.titles.get(self.index)
    }

    pub fn get_next_todo_type(&self) -> Option<&String> {
        let index = self.index % self.titles.len();
        self.titles.get(index)
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
