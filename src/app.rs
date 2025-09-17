use crate::{
    term::{Event, Terminal},
    tui::Tui,
};

#[derive(Debug)]
pub struct App {
    term: Terminal,
    tui: Tui,
}

impl App {
    pub fn build() -> anyhow::Result<Self> {
        Ok(Self {
            term: Terminal::new(),
            tui: Tui::build()?,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.term.start();
        self.tui.enter()?;

        tracing::info!("Application initialized.");
        while !self.tui.should_quit()
            && let Some(event) = self.term.next_event().await
        {
            match event {
                Event::Init => self.tui.init(),
                Event::Key(ke) => self.tui.handle_key(ke),
                Event::Resize(w, h) => self.tui.resize(w, h)?,
                Event::Tick(delta) => {
                    self.tui.tick(delta);
                    self.tui.render()?;
                }
                Event::FocusGained => {
                    self.term.start();
                    self.tui.resume()?;
                }
                Event::FocusLost => {
                    self.term.stop();
                    self.tui.suspend()?;
                }
            }
        }

        self.term.stop();
        self.tui.terminate();
        self.tui.exit()?;
        tracing::info!("Application terminated.");

        Ok(())
    }
}
