mod banner;
mod footer;
mod params;
mod style;
mod visualizer;

use std::io::Write;

use banner::Banner;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use footer::Footer;
use ratatui::{
    buffer::Buffer,
    crossterm::terminal,
    layout::{Constraint, Flex, Layout, Rect},
    prelude::CrosstermBackend,
    widgets::WidgetRef,
};
use signal_hook::{consts::signal, low_level};
use tokio::{sync::mpsc, time::Instant};
use visualizer::Visualizer;

trait ActiveWidget: WidgetRef {
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
    visualizer: Visualizer,
    footer: Footer,
}

impl Components {
    fn init(&mut self) {
        self.comps_mut(|comp| comp.init());
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [top, middle, bottom] = Layout::vertical([
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);
        let [_top_left, top_right] =
            Layout::horizontal([Constraint::Percentage(100), Constraint::Min(35)])
                .flex(Flex::Start)
                .areas(top);

        self.banner.render_ref(top_right, buf);
        self.visualizer.render_ref(middle, buf);
        self.footer.render_ref(bottom, buf);
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
        <[&mut dyn ActiveWidget; 3]>::into_iter([
            &mut self.banner,
            &mut self.visualizer,
            &mut self.footer,
        ])
        .for_each(f);
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum TuiMode {
    #[default]
    Params,
    Visualizer,
    Help,
}

impl TuiMode {
    #[inline]
    fn is_parameters(&self) -> bool {
        matches!(self, Self::Params)
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
    should_quit: bool,
    mode: TuiMode,
    _last_mode: TuiMode,
}

impl Tui {
    pub fn build() -> anyhow::Result<Self> {
        let (_sig_tx, sig_rx) = mpsc::unbounded_channel();
        Ok(Self {
            term: ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
            should_quit: false,
            mode: TuiMode::default(),
            _last_mode: TuiMode::default(),
            components: Components {
                banner: Banner,
                visualizer: Visualizer::new(sig_rx),
                footer: Footer,
            },
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

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.components
            .render(self.term.get_frame().area(), self.term.current_buffer_mut());
        self.flush()
    }

    #[inline]
    pub fn resize(&mut self, width: u16, height: u16) -> anyhow::Result<()> {
        self.term.resize(Rect::new(0, 0, width, height))?;
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if let KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        } = key
        {
            match code {
                KeyCode::Esc => self.should_quit = true,
                KeyCode::Tab => self.toggle_help(),
                KeyCode::Up if modifiers.contains(KeyModifiers::SHIFT) => {
                    self.swap_mode(TuiMode::Params);
                }
                KeyCode::Down if modifiers.contains(KeyModifiers::SHIFT) => {
                    self.swap_mode(TuiMode::Visualizer);
                }
                _ => match self.mode {
                    TuiMode::Params => {}
                    TuiMode::Visualizer => self.components.visualizer.handle_key(key),
                    TuiMode::Help => {}
                },
            }
        };
    }

    #[inline]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        self.term.flush()?;
        self.term.swap_buffers();
        self.term.backend_mut().flush()?;
        Ok(())
    }

    fn toggle_help(&mut self) {
        if self.mode.is_help() {
            self.swap_mode(self._last_mode);
        } else {
            self.swap_mode(TuiMode::Help);
        }
    }

    fn swap_mode(&mut self, mode: TuiMode) {
        self._last_mode = self.mode;
        self.mode = mode;
        tracing::debug!("TUI mode transitioned to {mode:?}");
    }
}
