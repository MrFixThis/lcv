use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
};

use super::{ActiveWidget, Ctx, style::Theme};

#[derive(Debug, Clone, Copy)]
pub(super) struct Footer;

impl Footer {
    const AUTHOR: &'static str = "MrFixThis";
}

impl ActiveWidget for Footer {
    fn render_ref(&self, _ctx: &Ctx<'_>, frame: &mut Frame<'_>, area: Rect) {
        let sign = Line::from_iter([
            Span::raw("with").style(Theme::TEXT),
            Span::raw(" <3").style(Theme::ERROR),
            Span::raw(" by ").style(Theme::TEXT),
            Span::raw(Self::AUTHOR).style(Theme::TITLE),
            Span::raw(" "),
        ])
        .right_aligned();

        let instructions = Line::from_iter([
            Span::raw(" <Esc>").style(Theme::HINT.add_modifier(Modifier::ITALIC)),
            Span::raw(" to quit").style(Theme::TEXT),
            Span::raw(" | ").style(Theme::BORDER_TERNARY),
            Span::raw("[<Shift>]<Tab>").patch_style(Theme::HINT),
            Span::raw(" to swap section").style(Theme::TEXT),
            Span::raw(" | ").style(Theme::BORDER_TERNARY),
            Span::raw("<Left/Right>").patch_style(Theme::HINT),
            Span::raw(" to scroll waveform").style(Theme::TEXT),
        ])
        .left_aligned();

        frame.render_widget(instructions, area);
        frame.render_widget(sign, area);
    }
}
