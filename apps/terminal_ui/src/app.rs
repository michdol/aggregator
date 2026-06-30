use tokio::sync::mpsc;

#[derive(Debug)]
pub struct App {
    pub counter: u8,
    pub should_quit: bool,
    pub last_message: String,
    receiver: mpsc::Receiver<String>,
}

impl App {
    pub fn new(receiver: mpsc::Receiver<String>) -> Self {
        Self {
            counter: 0,
            last_message: String::from(""),
            should_quit: false,
            receiver: receiver,
        }
    }

    pub async fn tick(&mut self) {
        if let Some(data) = self.receiver.recv().await {
            self.last_message = data;
        }
    }

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

    pub async fn read_from_receiver(&mut self) {
        if let Some(data) = self.receiver.recv().await {
            self.last_message = data;
        }
    }
}
