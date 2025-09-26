mod banner;
mod footer;
mod params;
mod style;
mod visualizer;

use std::marker::PhantomData;

use banner::Banner;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use footer::Footer;
use params::Parameters;
use ratatui::{
    Frame,
    crossterm::terminal,
    layout::{Constraint, Flex, Layout, Rect},
    prelude::CrosstermBackend,
};
use signal_hook::{consts::signal, low_level};
use tokio::{sync::mpsc, time::Instant};
use visualizer::Visualizer;

trait ActiveWidget {
    fn init(&mut self) {}

    fn terminate(&mut self) {}

    fn render_ref(&self, ctx: &Ctx<'_>, frame: &mut Frame<'_>, area: Rect);

    fn handle_key(&mut self, key: KeyEvent) {
        let _ = key;
    }

    fn tick(&mut self, delta: Instant) {
        let _ = delta;
    }
}

#[derive(Debug, Clone)]
struct Ctx<'a> {
    mode: TuiMode,
    _phantom: PhantomData<&'a ()>,
}

#[derive(Debug, Default, Clone, Copy)]
enum TuiMode {
    #[default]
    Params,
    Visualizer,
}

impl TuiMode {
    #[inline]
    fn is_params(&self) -> bool {
        matches!(self, Self::Params)
    }

    #[inline]
    fn is_visualizer(&self) -> bool {
        matches!(self, Self::Visualizer)
    }
}

#[derive(Debug)]
pub struct Tui {
    banner: Banner,
    params: Parameters,
    visualizer: Visualizer,
    footer: Footer,
    mode: TuiMode,
    should_quit: bool,
    term: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Tui {
    pub fn build() -> anyhow::Result<Self> {
        let (sig_tx, sig_rx) = mpsc::unbounded_channel();
        Ok(Self {
            term: ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
            mode: TuiMode::default(),
            should_quit: false,
            params: Parameters::new(sig_tx),
            banner: Banner,
            visualizer: Visualizer::new(sig_rx),
            footer: Footer,
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
        self.comps_mut(|comp| comp.init());
    }

    #[inline]
    pub fn terminate(&mut self) {
        self.comps_mut(|comp| comp.terminate());
    }

    #[inline]
    pub fn tick(&mut self, delta: Instant) {
        self.comps_mut(|comp| comp.tick(delta));
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.term.draw(|frame| {
            let [top, middle, bottom] = Layout::vertical([
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .areas(frame.area());
            let [top_left, top_right] =
                Layout::horizontal([Constraint::Percentage(100), Constraint::Min(35)])
                    .flex(Flex::Start)
                    .areas(top);

            let ctx = Ctx {
                mode: self.mode,
                _phantom: PhantomData,
            };

            self.params.render_ref(&ctx, frame, top_left);
            self.banner.render_ref(&ctx, frame, top_right);
            self.visualizer.render_ref(&ctx, frame, middle);
            self.footer.render_ref(&ctx, frame, bottom);
        })?;

        Ok(())
    }

    #[inline]
    pub fn resize(&mut self, width: u16, height: u16) -> anyhow::Result<()> {
        self.term.resize(Rect::new(0, 0, width, height))?;
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if let KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        } = key
        {
            match code {
                KeyCode::Esc => self.should_quit = true,
                KeyCode::BackTab => self.swap_mode(TuiMode::Params),
                KeyCode::Tab => self.swap_mode(TuiMode::Visualizer),
                _ => match self.mode {
                    TuiMode::Params => self.params.handle_key(key),
                    TuiMode::Visualizer => self.visualizer.handle_key(key),
                },
            }
        };
    }

    #[inline]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    #[inline]
    fn swap_mode(&mut self, mode: TuiMode) {
        self.mode = mode;
        tracing::debug!("TUI mode transitioned to {mode:?}");
    }

    fn comps_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut dyn ActiveWidget),
    {
        <[&mut dyn ActiveWidget; 4]>::into_iter([
            &mut self.banner,
            &mut self.params,
            &mut self.visualizer,
            &mut self.footer,
        ])
        .for_each(f);
    }
}
