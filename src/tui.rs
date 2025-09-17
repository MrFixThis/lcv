pub mod banner;
pub mod style;
pub mod widget;

use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use std::sync::Arc;
use tokio::{
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    task::JoinHandle,
    time::Instant,
};

use crate::term::{Event, Terminal};

pub enum Action {
    Init,
    Quit,
    Tick(Instant),
    Key(KeyEvent),
    Paste(String),
}

#[derive(Debug)]
pub struct App {
    term: Arc<Mutex<Terminal>>,
    map_handle: JoinHandle<anyhow::Result<()>>,
    act_tx: UnboundedSender<Action>,
    act_rx: UnboundedReceiver<Action>,
}

impl App {
    pub fn build() -> anyhow::Result<Self> {
        let (act_tx, act_rx) = mpsc::unbounded_channel();
        Ok(Self {
            term: Arc::new(Mutex::new(Terminal::build()?)),
            map_handle: tokio::spawn(async move { Ok(()) }),
            act_tx,
            act_rx,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        tracing::info!("TUI initialized.");
        self.map_handle = self.spawn_mapper();

        while let Some(action) = self.act_rx.recv().await {
            match action {
                Action::Quit => self.quit().await?,
                Action::Init => todo!(),
                Action::Tick(delta) => todo!(),
                Action::Key(ke) => todo!(),
                Action::Paste(_) => {},
            }
        }

        Ok(())
    }

    async fn quit(&self) -> anyhow::Result<()> {
        self.term.lock().await.exit()?;
        if !self.map_handle.is_finished() {
            self.map_handle.abort();
        }
        tracing::info!("TUI disposed.");
        Ok(())
    }

    fn spawn_mapper(&self) -> JoinHandle<anyhow::Result<()>> {
        let act_tx = self.act_tx.clone();
        let term = Arc::clone(&self.term);

        tokio::spawn(async move {
            loop {
                let mut term = term.lock().await;
                if let Some(event) = term.next_event().await {
                    match event {
                        Event::Init => act_tx.send(Action::Init)?,
                        Event::Tick(delta) => act_tx.send(Action::Tick(delta))?,
                        Event::Key(ke) => act_tx.send(Action::Key(ke))?,
                        Event::Paste(cont) => act_tx.send(Action::Paste(cont))?,
                        Event::Resize(w, h) => term.resize(Rect::new(0, 0, w, h))?,
                        Event::FocusGained => term.resume()?,
                        Event::FocusLost => term.suspend()?,
                        Event::Quit => break,
                    }
                }
            }

            Ok(())
        })
    }
}
