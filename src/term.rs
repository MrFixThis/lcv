use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{DisableBracketedPaste, EnableBracketedPaste, Event as CTEvent, EventStream, KeyEvent},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::stream::StreamExt;
use ratatui::{Terminal as Inner, backend::CrosstermBackend};
use signal_hook::{consts::signal, low_level};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::Instant,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Init,
    Quit,
    FocusGained,
    FocusLost,
    Tick(Instant),
    Resize(u16, u16),
    Key(KeyEvent),
    Paste(String),
}

#[derive(Debug)]
pub struct Terminal {
    inner: Inner<CrosstermBackend<std::io::Stdout>>,
    task_handle: JoinHandle<anyhow::Result<()>>,
    cancel_tok: CancellationToken,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
}

impl Deref for Terminal {
    type Target = Inner<CrosstermBackend<std::io::Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Terminal {
    const GLOB_TICK_RATE: f64 = 6.0;

    pub fn build() -> anyhow::Result<Self> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Ok(Self {
            event_rx,
            event_tx,
            inner: Inner::new(CrosstermBackend::new(std::io::stdout()))?,
            task_handle: tokio::spawn(async { Ok(()) }),
            cancel_tok: CancellationToken::new(),
        })
    }

    #[inline(always)]
    pub async fn next_event(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }

    pub fn enter(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stdout(),
            EnterAlternateScreen,
            cursor::Hide,
            EnableBracketedPaste,
        )?;
        tracing::info!("Terminal transitioned to raw mode.");
        self.start();

        Ok(())
    }

    pub fn exit(&mut self) -> anyhow::Result<()> {
        self.stop();
        if terminal::is_raw_mode_enabled()? {
            self.inner.flush()?;
            crossterm::execute!(
                std::io::stdout(),
                DisableBracketedPaste,
                LeaveAlternateScreen,
                cursor::Show,
            )?;

            terminal::disable_raw_mode()?;
            tracing::info!("Terminal transitioned to normal mode.");
        }

        Ok(())
    }

    #[inline]
    pub fn resume(&mut self) -> anyhow::Result<()> {
        #[cfg(windows)]
        self.enter()?;
        Ok(())
    }

    #[inline]
    pub fn suspend(&mut self) -> anyhow::Result<()> {
        self.exit()?;
        #[cfg(not(windows))]
        low_level::raise(signal::SIGTSTP)?;
        Ok(())
    }

    fn start(&mut self) {
        self.stop();
        self.cancel_tok = CancellationToken::new();

        let cancel_tok = self.cancel_tok.clone();
        let event_tx = self.event_tx.clone();
        let mut stream = EventStream::new();
        let mut tick_interval =
            tokio::time::interval(Duration::from_secs_f64(1.0 / Self::GLOB_TICK_RATE));

        self.task_handle = tokio::spawn(async move {
            tracing::info!("Terminal event loop initialized.");
            event_tx.send(Event::Init)?;

            loop {
                tokio::select! {
                    delta = tick_interval.tick() => event_tx.send(Event::Tick(delta))?,
                    _ = cancel_tok.cancelled() => {
                        event_tx.send(Event::Quit)?;
                        break;
                    },
                    Some(result) = stream.next() => match result {
                        Err(err) => tracing::error!("Error capturing event from terminal: {err}"),
                        Ok(event) => match event {
                            CTEvent::FocusGained => event_tx.send(Event::FocusGained)?,
                            CTEvent::FocusLost => event_tx.send(Event::FocusLost)?,
                            CTEvent::Key(ke) => event_tx.send(Event::Key(ke))?,
                            CTEvent::Paste(cont) => event_tx.send(Event::Paste(cont))?,
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
    fn stop(&self) {
        self.cancel_tok.cancel();
        self.task_handle.abort();
        tracing::info!("Terminal event loop terminated.");
    }
}
