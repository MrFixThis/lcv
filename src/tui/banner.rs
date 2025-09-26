use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::tui::style::Theme;

use super::{ActiveWidget, Ctx};

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

impl ActiveWidget for Banner {
    fn render_ref(&self, _ctx: &Ctx<'_>, frame: &mut Frame<'_>, area: Rect) {
        let title = Span::raw(format!(" v{} ", env!("CARGO_PKG_VERSION")));
        let footer = Span::raw(format!(" {} ", env!("CARGO_PKG_DESCRIPTION")));

        let block = Block::new()
            .title_top(Line::from(title).left_aligned().style(Theme::SUB_TITLE))
            .title_bottom(Line::from(footer).right_aligned().style(Theme::HINT))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .style(Theme::BORDER_PRIMARY);

        frame.render_widget(
            Paragraph::new(Text::from(Self::LOGO).style(Theme::TEXT))
                .block(block)
                .centered(),
            area,
        );
    }
}
