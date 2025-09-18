use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, WidgetRef},
};

use crate::tui::style::Theme;

use super::ActiveWidget;

#[derive(Debug, Clone, Copy)]
pub(super) struct Banner;

impl Banner {
    const LOGO: &'static str = r"
██╗      ██████╗██╗   ██╗
██║     ██╔════╝██║   ██║
██║     ██║     ██║   ██║
██║     ██║     ╚██╗ ██╔╝
███████╗╚██████╗ ╚████╔╝
╚══════╝ ╚═════╝  ╚═══╝  
";
}

impl WidgetRef for Banner {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let title = Span::raw(format!(" v{} ", env!("CARGO_PKG_VERSION")));
        let footer = Span::raw(format!(" {} ", env!("CARGO_PKG_DESCRIPTION")));

        let block = Block::new()
            .title_top(Line::from(title).left_aligned().style(Theme::HINT))
            .title_bottom(Line::from(footer).right_aligned().style(Theme::HINT))
            .borders(Borders::ALL)
            .style(Theme::BORDER_PRIMARY);

        Paragraph::new(Text::from(Self::LOGO).style(Theme::TEXT))
            .block(block)
            .centered()
            .render(area, buf);
    }
}

impl ActiveWidget for Banner {}
