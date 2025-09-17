pub mod banner;
pub mod style;

use crossterm::{
    cursor,
    event::KeyEvent,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{crossterm::terminal, layout::Rect, prelude::CrosstermBackend, widgets::WidgetRef};
use signal_hook::{consts::signal, low_level};
use tokio::time::Instant;

pub trait ActiveWidget: WidgetRef {
    fn init(&mut self) {}

    fn terminate(&mut self) {}

    fn handle_key(&mut self, key: KeyEvent) {
        let _ = key;
    }

    fn tick(&mut self, delta: Instant) {
        let _ = delta;
    }

    #[inline]
    fn boxed(self) -> Box<dyn WidgetRef + 'static>
    where
        Self: Sized + 'static,
    {
        Box::from(self)
    }
}

#[derive(Debug)]
pub struct Tui {
    term: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
    should_quit: bool,
}

impl Tui {
    pub fn build() -> anyhow::Result<Self> {
        Ok(Self {
            term: ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
            should_quit: false,
        })
    }

    pub fn enter(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;
        tracing::info!("Terminal transitioned to raw mode.");

        Ok(())
    }

    pub fn exit(&mut self) -> anyhow::Result<()> {
        if terminal::is_raw_mode_enabled()? {
            self.term.flush()?;
            crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show)?;
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

    pub fn suspend(&mut self) -> anyhow::Result<()> {
        self.exit()?;
        #[cfg(not(windows))]
        low_level::raise(signal::SIGTSTP)?;
        Ok(())
    }

    pub fn init(&mut self) {}

    pub fn terminate(&mut self) {}

    pub fn handle_key(&mut self, key: KeyEvent) {
        let _ = key;
    }

    pub fn tick(&mut self, delta: Instant) {
        let _ = delta;
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        // TODO: Render here
        self.term.flush()?;
        Ok(())
    }

    #[inline]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    #[inline]
    pub fn resize(&mut self, width: u16, height: u16) -> anyhow::Result<()> {
        self.term.resize(Rect::new(0, 0, width, height))?;
        Ok(())
    }
}
