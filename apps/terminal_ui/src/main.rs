// https://ratatui.rs/tutorials/counter-app/_multiple-files/event/
use color_eyre::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::App;
use events::{Event, EventHandler};
use tui::Tui;
use update::update;
// App.
pub mod app;
// Terminal events handler.
pub mod events;
// Widget renderer.
pub mod ui;
// Terminal user interface.
pub mod tui;
// App updated.
pub mod update;

fn main() -> Result<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
    tui.exit()?;
    Ok(())
}
