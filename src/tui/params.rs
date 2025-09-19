use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};
use strum_macros::{AsRefStr, FromRepr};
use tui_input::Input as Inner;

use crate::coder::LineCoder;

use super::ActiveWidget;

#[derive(Clone, Default, Copy, AsRefStr, FromRepr)]
enum CoderName {
    #[default]
    #[strum(serialize = "NRZ-L")]
    Nrzl,
    #[strum(serialize = "NRZI")]
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

impl WidgetRef for CoderName {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {}
}

impl ActiveWidget for CoderName {}

struct Input {
    inner: Inner,
    is_valid: bool,
}

impl WidgetRef for Input {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {}
}

impl ActiveWidget for Input {}

pub(super) struct Parameters {
    coder_name: CoderName,
    raw_coder: Box<dyn LineCoder>,
    bits_input: Input,
    tb_field: Input,
    v_field: Input,
    duty_field: Option<Input>,
}

impl Parameters {}
