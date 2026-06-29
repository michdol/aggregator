#[derive(Debug, Default)]
pub struct App {
    pub counter: u8,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }
}
