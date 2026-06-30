// https://ratatui.rs/tutorials/counter-app/_multiple-files/event/
use color_eyre::Result;
use futures_util::stream::StreamExt;
use lapin::options::BasicAckOptions;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc;

use shared_models::RabbitMq;

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

#[tokio::main]
async fn main() -> Result<()> {
    let (sender, receiver) = mpsc::channel::<String>(32);

    let mut app = App::new(receiver);
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let amqp_url = "amqp://localhost:5672/%2f";
    let rabbit = match RabbitMq::new(amqp_url, "trucks").await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed instantiating rabbit {}", err);
        }
    };

    tokio::spawn(async move {
        if let Ok(mut consumer) = rabbit.get_consumer().await {
            while let Some(message) = consumer.next().await {
                if let Ok(message) = message {
                    let binary = &message.data;
                    if let Ok(s) = str::from_utf8(&binary) {
                        if sender.send(String::from(s)).await.is_err() {
                            println!("Receiver was dropped. exiting background task");
                            return;
                        }
                    }
                    if let Err(e) = message.ack(BasicAckOptions::default()).await {
                        eprintln!("Error acking {:?}", e);
                    }
                }
            }
        }
    });

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => app.tick().await,
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
    tui.exit()?;
    Ok(())
}
