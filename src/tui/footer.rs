use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::WidgetRef,
};

use super::{ActiveWidget, style::Theme};

#[derive(Debug, Clone, Copy)]
pub(super) struct Footer;

impl Footer {
    const AUTHOR: &'static str = "MrFixThis";
}

impl WidgetRef for Footer {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let sign = Line::from_iter([
            Span::raw("with").style(Theme::TEXT),
            Span::raw(" <3").style(Theme::WARN),
            Span::raw(" by ").style(Theme::TEXT),
            Span::raw(Self::AUTHOR).style(Theme::TITLE),
            Span::raw(" "),
        ])
        .right_aligned();

        let instructions = Line::from_iter([
            Span::raw(" <Esc>").style(Theme::HINT),
            Span::raw(" to quit").style(Theme::TEXT),
            Span::raw(" | ").style(Theme::BORDER_SECONDARY),
            Span::raw("<Tab>").patch_style(Theme::HINT),
            Span::raw(" to toggle help").style(Theme::TEXT),
            Span::raw(" | ").style(Theme::BORDER_SECONDARY),
            Span::raw("<Left/Right>").patch_style(Theme::HINT),
            Span::raw(" & ").style(Theme::TEXT),
            Span::raw("<Shift><Up/Down>").patch_style(Theme::HINT),
            Span::raw(" to naviate").style(Theme::TEXT),
        ])
        .left_aligned();

        let layout = Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(area);
        instructions.render_ref(layout[0], buf);
        sign.render_ref(layout[1], buf);
    }
}

impl ActiveWidget for Footer {}
