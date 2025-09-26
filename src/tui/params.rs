use std::{cell::RefCell, fmt::Debug};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    text::{Line, Span},
    widgets::{
        Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState,
    },
};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, FromRepr};
use tokio::sync::mpsc::UnboundedSender;
use tui_input::{Input, InputRequest};

use crate::{
    coder::{
        LineCoder, SigElement,
        ami::Ami,
        hdb3::Hdb3,
        manch::Manchester,
        mlt3::Mlt3,
        nrz::{Nrzi, Nrzl},
        rz::Rz,
    },
    util,
};

use super::{ActiveWidget, Ctx, style::Theme};

#[derive(Debug, Default, Clone, Copy)]
enum Mode {
    #[default]
    Normal,
    Insert,
}

#[derive(Debug, Default, Clone, Copy, AsRefStr)]
enum Focus {
    Method,
    #[default]
    Bits,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, EnumIter, AsRefStr, FromRepr)]
enum CoderName {
    #[default]
    #[strum(serialize = "NRZ-L")]
    Nrzl,
    #[strum(serialize = "NRZ-I")]
    Nrzi,
    #[strum(serialize = "RZ")]
    Rz,
    #[strum(serialize = "Manchester 802.3")]
    Manchester,
    #[strum(serialize = "HDB3")]
    Hdb3,
    #[strum(serialize = "MLT-3")]
    Mlt3,
    #[strum(serialize = "AMI")]
    Ami,
}

impl CoderName {
    fn to_raw(self) -> Box<dyn LineCoder> {
        match self {
            CoderName::Nrzl => Nrzl::new().boxed(),
            CoderName::Nrzi => Nrzi::new().boxed(),
            CoderName::Rz => Rz::new().boxed(),
            CoderName::Manchester => Manchester::new().boxed(),
            CoderName::Hdb3 => Hdb3::new().boxed(),
            CoderName::Mlt3 => Mlt3::new().boxed(),
            CoderName::Ami => Ami::new().boxed(),
        }
    }
}

pub(super) struct Parameters {
    mode: Mode,
    focus: Focus,
    coder_name: CoderName,
    raw_coder: Box<dyn LineCoder>,
    bits_input: Input,
    scroll_state: RefCell<ScrollbarState>,
    list_state: RefCell<ListState>,
    sig_tx: UnboundedSender<Box<[SigElement]>>,
}

impl Debug for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parameters")
            .field("mode", &self.mode)
            .field("focus", &self.focus)
            .field("coder", &self.coder_name)
            .field("bits_input", &self.bits_input)
            .field("sig_tx", &self.sig_tx)
            .finish()
    }
}

impl Parameters {
    pub(super) fn new(sig_tx: UnboundedSender<Box<[SigElement]>>) -> Self {
        let coder_name = CoderName::default();
        Self {
            sig_tx,
            coder_name,
            raw_coder: coder_name.to_raw(),
            mode: Default::default(),
            focus: Default::default(),
            bits_input: Default::default(),
            scroll_state: RefCell::new(ScrollbarState::new(CoderName::iter().count())),
            list_state: RefCell::new(ListState::default().with_selected(Some(0))),
        }
    }

    fn handle_key_normal(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Enter if matches!(self.focus, Focus::Bits) => self.mode = Mode::Insert,
            KeyCode::Left => self.focus = Focus::Method,
            KeyCode::Right => self.focus = Focus::Bits,
            KeyCode::Up if matches!(self.focus, Focus::Method) => self.prev_coder(),
            KeyCode::Down if matches!(self.focus, Focus::Method) => self.next_coder(),
            _ => {}
        }
    }

    fn handle_key_insert(&mut self, key_code: KeyCode, modifiers: KeyModifiers) {
        match key_code {
            KeyCode::Enter => self.mode = Mode::Normal,
            KeyCode::Left => _ = self.bits_input.handle(InputRequest::GoToPrevChar),
            KeyCode::Right => _ = self.bits_input.handle(InputRequest::GoToNextChar),
            KeyCode::Home => _ = self.bits_input.handle(InputRequest::GoToStart),
            KeyCode::End => _ = self.bits_input.handle(InputRequest::GoToEnd),
            KeyCode::Char(ch @ '0' | ch @ '1') => {
                _ = self.bits_input.handle(InputRequest::InsertChar(ch));
                self.parse_and_send();
            }
            KeyCode::Backspace if modifiers.contains(KeyModifiers::CONTROL) => {
                _ = self.bits_input.handle(InputRequest::DeletePrevWord);
                self.parse_and_send();
            }
            KeyCode::Backspace => {
                _ = self.bits_input.handle(InputRequest::DeletePrevChar);
                self.parse_and_send();
            }
            KeyCode::Delete if modifiers.contains(KeyModifiers::CONTROL) => {
                _ = self.bits_input.handle(InputRequest::DeleteNextWord);
                self.parse_and_send();
            }
            KeyCode::Delete => {
                _ = self.bits_input.handle(InputRequest::DeleteNextChar);
                self.parse_and_send();
            }
            _ => {}
        }
    }

    fn prev_coder(&mut self) {
        let Some(name) = CoderName::from_repr((self.coder_name as usize).saturating_sub(1)) else {
            return;
        };

        self.coder_name = name;
        self.raw_coder = name.to_raw();
        self.scroll_state.borrow_mut().prev();
        self.list_state.borrow_mut().select_previous();
        self.parse_and_send();
    }

    fn next_coder(&mut self) {
        let Some(name) = CoderName::from_repr(self.coder_name as usize + 1) else {
            return;
        };

        self.coder_name = name;
        self.raw_coder = name.to_raw();
        self.scroll_state.borrow_mut().next();
        self.list_state.borrow_mut().select_next();
        self.parse_and_send();
    }

    fn parse_and_send(&mut self) {
        self.sig_tx
            .send(
                self.raw_coder
                    .encode(&util::parse_bits(self.bits_input.value()).unwrap_or_default()),
            )
            .unwrap();
    }
}

impl ActiveWidget for Parameters {
    fn render_ref(&self, ctx: &Ctx<'_>, frame: &mut Frame<'_>, area: Rect) {
        let base = Block::bordered()
            .style(Theme::BORDER_PRIMARY)
            .border_type(if ctx.mode.is_params() {
                BorderType::Double
            } else {
                BorderType::Rounded
            })
            .title(
                Line::from_iter([
                    Span::raw("[ ").style(Theme::BORDER_PRIMARY),
                    Span::raw("Parameters").style(Theme::WARN),
                    Span::raw(" ]").style(Theme::BORDER_PRIMARY),
                ])
                .alignment(Alignment::Center),
            );
        frame.render_widget(base, area);

        let ((meth_sty, meth_border), (input_sty, input_border)) =
            if matches!(self.focus, Focus::Method) {
                (
                    (Theme::BORDER_SECONDARY, BorderType::Thick),
                    (Theme::BORDER_PRIMARY, BorderType::Rounded),
                )
            } else {
                (
                    (Theme::BORDER_PRIMARY, BorderType::Rounded),
                    (Theme::BORDER_SECONDARY, BorderType::Thick),
                )
            };

        let [left, right] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).areas(
                area.inner(Margin {
                    horizontal: 2,
                    vertical: 1,
                }),
            );

        let list =
            List::new(CoderName::iter().map(|name| {
                ListItem::new(Line::from(name.as_ref().to_owned())).style(Theme::TEXT)
            }))
            .highlight_style(Theme::HIGHLIGHT_ITEM)
            .highlight_symbol("> ")
            .block(
                Block::bordered()
                    .border_type(meth_border)
                    .style(meth_sty)
                    .title(Line::from(" Method ").style(Theme::SUB_TITLE)),
            );
        frame.render_stateful_widget(list, left, &mut self.list_state.borrow_mut());
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight).style(meth_sty),
            left,
            &mut self.scroll_state.borrow_mut(),
        );

        let [bits, help] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(2)]).areas(right);
        let input_rect = Rect::new(bits.x, bits.y + 1, bits.width, area.height / 3).inner(Margin {
            horizontal: 1,
            vertical: 0,
        });
        let input_scroll = self
            .bits_input
            .visual_scroll(input_rect.width.saturating_sub(3).max(1) as _);

        let bits_input = Paragraph::new(
            Line::from(self.bits_input.value())
                .style(Theme::TEXT)
                .alignment(Alignment::Left),
        )
        .scroll((0, input_scroll as _))
        .block(
            Block::bordered()
                .title(Line::from(" Bits ").style(Theme::SUB_TITLE))
                .padding(Padding::horizontal(1))
                .border_type(input_border)
                .border_style(if matches!(self.mode, Mode::Insert) {
                    Theme::BORDER_TERNARY
                } else {
                    input_sty
                }),
        );
        frame.render_widget(bits_input, input_rect);

        if matches!(self.mode, Mode::Insert) {
            let x = self.bits_input.visual_cursor().max(input_scroll) - input_scroll + 2;
            frame.set_cursor_position((input_rect.x + x as u16, input_rect.y + 1));
        }

        let mut help_txt = vec![
            Span::raw("<Left/Right>").patch_style(Theme::HINT),
            Span::raw(" to navigate").style(Theme::TEXT),
            Span::raw(" | ").style(Theme::BORDER_TERNARY),
        ];

        help_txt.extend(if matches!(self.focus, Focus::Bits) {
            [
                Span::raw("<Enter>").style(Theme::HINT),
                Span::raw(" to toggle insert mode").style(Theme::TEXT),
            ]
        } else {
            [
                Span::raw("<Up/Down>").patch_style(Theme::HINT),
                Span::raw(" to select a method").style(Theme::TEXT),
            ]
        });
        frame.render_widget(Line::from_iter(help_txt).centered(), help);
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if let KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        } = key
        {
            match self.mode {
                Mode::Normal => self.handle_key_normal(code),
                Mode::Insert => self.handle_key_insert(code, modifiers),
            }
        }
    }
}
