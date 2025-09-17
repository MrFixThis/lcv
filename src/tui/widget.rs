use crossterm::event::KeyEvent;
use ratatui::widgets::WidgetRef;
use tokio::time::Instant;

pub trait ActiveWidget: WidgetRef {
    fn init(&mut self) {}

    fn handle_key(&mut self, key: KeyEvent) {
        let _ = key;
    }

    fn on_tick(&mut self, delta: Instant) {
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
