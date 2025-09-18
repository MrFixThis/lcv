pub mod banner;
pub mod style;

use std::io::Write;

use banner::Banner;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    crossterm::terminal,
    layout::{Constraint, Flex, Layout, Rect},
    prelude::CrosstermBackend,
    widgets::WidgetRef,
};
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
}

#[derive(Debug)]
struct Components {
    banner: Banner,
}

impl Components {
    fn init(&mut self) {
        self.comps_mut(|comp| comp.init());
    }

    fn terminate(&mut self) {
        self.comps_mut(|comp| comp.terminate());
    }

    fn tick(&mut self, delta: Instant) {
        self.comps_mut(|comp| comp.tick(delta));
    }

    fn comps_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut dyn ActiveWidget),
    {
        <[&mut dyn ActiveWidget; 1]>::into_iter([&mut self.banner]).for_each(f);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TuiMode {
    #[default]
    Parameters,
    Visualizer,
    Help,
}

impl TuiMode {
    #[inline]
    fn is_parameters(&self) -> bool {
        matches!(self, Self::Parameters)
    }

    #[inline]
    fn is_visualizer(&self) -> bool {
        matches!(self, Self::Visualizer)
    }

    #[inline]
    fn is_help(&self) -> bool {
        matches!(self, Self::Help)
    }
}

#[derive(Debug)]
pub struct Tui {
    term: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
    components: Components,
    mode: TuiMode,
    should_quit: bool,
}

impl Tui {
    pub fn build() -> anyhow::Result<Self> {
        Ok(Self {
            term: ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
            mode: TuiMode::default(),
            should_quit: false,
            components: Components { banner: Banner },
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

    #[inline]
    pub fn init(&mut self) {
        self.components.init();
    }

    #[inline]
    pub fn terminate(&mut self) {
        self.components.terminate();
    }

    #[inline]
    pub fn tick(&mut self, delta: Instant) {
        self.components.tick(delta);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if let KeyEvent {
            code: KeyCode::Char('q'),
            kind: KeyEventKind::Press,
            ..
        } = key
        {
            self.should_quit = true;
            return;
        };

        match self.mode {
            TuiMode::Parameters => {}
            TuiMode::Visualizer => {}
            TuiMode::Help => {}
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let chunks = Layout::vertical([
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(self.term.get_frame().area());
        let upper_chunks = Layout::horizontal([Constraint::Percentage(100), Constraint::Min(35)])
            .flex(Flex::Start)
            .split(chunks[0]);
        let buf = self.term.current_buffer_mut();

        self.components.banner.render_ref(upper_chunks[1], buf);

        self.flush()
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

    fn flush(&mut self) -> anyhow::Result<()> {
        self.term.flush()?;
        self.term.swap_buffers();
        self.term.backend_mut().flush()?;
        Ok(())
    }
}
