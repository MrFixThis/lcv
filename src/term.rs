use std::time::Duration;

use crossterm::event::{Event as CTEvent, EventStream, KeyEvent};
use futures::stream::StreamExt;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::Instant,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Init,
    FocusGained,
    FocusLost,
    Tick(Instant),
    Resize(u16, u16),
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct Terminal {
    task_handle: JoinHandle<anyhow::Result<()>>,
    cancel_tok: CancellationToken,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
}

impl Terminal {
    const GLOB_TICK_RATE: u64 = 5;

    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Self {
            event_rx,
            event_tx,
            task_handle: tokio::spawn(async { Ok(()) }),
            cancel_tok: CancellationToken::new(),
        }
    }

    #[inline(always)]
    pub async fn next_event(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }

    pub fn start(&mut self) {
        self.stop();
        self.cancel_tok = CancellationToken::new();

        let cancel_tok = self.cancel_tok.clone();
        let event_tx = self.event_tx.clone();
        let mut stream = EventStream::new();
        let mut tick_interval = tokio::time::interval(Duration::from_millis(Self::GLOB_TICK_RATE));

        self.task_handle = tokio::spawn(async move {
            tracing::info!("Terminal event loop initialized.");
            event_tx.send(Event::Init)?;

            loop {
                tokio::select! {
                    _ = cancel_tok.cancelled() => break,
                    delta = tick_interval.tick() => event_tx.send(Event::Tick(delta))?,
                    Some(result) = stream.next() => match result {
                        Err(err) => tracing::error!("Error capturing event from terminal: {err}"),
                        Ok(event) => match event {
                            CTEvent::FocusGained => event_tx.send(Event::FocusGained)?,
                            CTEvent::FocusLost => event_tx.send(Event::FocusLost)?,
                            CTEvent::Key(ke) => event_tx.send(Event::Key(ke))?,
                            CTEvent::Resize(a, b) => event_tx.send(Event::Resize(a, b))?,
                            _ => {}
                        }
                    }
                }
            }

            Ok(())
        });
    }

    #[inline]
    pub fn stop(&self) {
        self.cancel_tok.cancel();
        self.task_handle.abort();
        tracing::info!("Terminal event loop terminated.");
    }
}
