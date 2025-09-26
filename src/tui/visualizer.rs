use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Chart, Dataset, GraphType},
};
use tokio::{sync::mpsc::UnboundedReceiver, time::Instant};

use crate::coder::SigElement;

use super::{ActiveWidget, Ctx, style::Theme};

#[derive(Debug)]
pub(super) struct Visualizer {
    sig_elems: Box<[SigElement]>,
    points: Box<[(f64, f64)]>,
    start_pos: usize,
    sig_rx: UnboundedReceiver<Box<[SigElement]>>,
}

impl Visualizer {
    const MAX_POINTS_TO_RENDER: usize = 35;

    pub(super) fn new(sig_rx: UnboundedReceiver<Box<[SigElement]>>) -> Self {
        Self {
            start_pos: 0,
            points: Default::default(),
            sig_elems: Default::default(),
            sig_rx,
        }
    }

    fn right(&mut self) {
        let n_vis = Self::MAX_POINTS_TO_RENDER.min(self.points.len().max(1));
        let max_start = self.points.len().saturating_sub(n_vis);
        self.start_pos = (self.start_pos + 1).min(max_start);
    }

    fn left(&mut self) {
        self.start_pos = self.start_pos.saturating_sub(1);
    }

    fn create_axis(&self) -> (Axis<'_>, Axis<'_>, [(f64, f64); 2]) {
        let bounds @ [(x0, x1), (y0, y1)] = self.find_axis_bounds();
        let x_axis = Axis::default()
            .title(Line::from_iter([
                Span::from("Time ").style(Theme::SUB_TITLE),
                Span::from("(sec)").style(Theme::WARN.italic()),
            ]))
            .bounds([x0, x1])
            .style(Theme::BORDER_TERNARY)
            .labels([
                Line::from(format!("{x0:.1}")).style(Theme::HINT),
                Line::from(format!("{:.1}", (x0 + x1) / 2.0)).style(Theme::HINT),
                Line::from(format!("{x1:.1}")).style(Theme::HINT),
            ]);

        let y_axis = Axis::default()
            .title(Line::from("Voltage").style(Theme::SUB_TITLE))
            .bounds([y0, y1])
            .style(Theme::BORDER_TERNARY)
            .labels(if y0 < 0.0 && y1 > 0.0 {
                [
                    Line::from("-V").style(Theme::HINT),
                    Line::from("0").style(Theme::HINT),
                    Line::from("+V").style(Theme::HINT),
                ]
            } else {
                [
                    Line::from("0").style(Theme::HINT),
                    Line::from("").style(Theme::HINT),
                    Line::from("+V").style(Theme::HINT),
                ]
            });

        (x_axis, y_axis, bounds)
    }

    fn find_axis_bounds(&self) -> [(f64, f64); 2] {
        let total = self.points.len();
        let n_vis = Self::MAX_POINTS_TO_RENDER.min(total.max(1));
        let start = self.start_pos.min(total.saturating_sub(n_vis));
        let slice = &self.points[start.saturating_sub(1)..(start + n_vis).min(total)];
        let x0 = if total > 0 { self.points[start].0 } else { 0.0 };
        let x1 = slice.last().map(|p| p.0).unwrap_or(1.0);

        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        for &(_, y) in slice {
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }
        if !y_min.is_finite() || !y_max.is_finite() || (y_max - y_min).abs() < 1e-12 {
            y_min = 0.0;
            y_max = 1.0;
        }

        let pad_sym = |m: f64| (m * 0.1).max(1e-6);
        let pad_pos = |span: f64| (span * 0.1).max(1e-6);
        let (y0, y1) = if y_min < 0.0 && y_max > 0.0 {
            let m = (y_max.abs().max(y_min.abs())).max(1.0)
                + pad_sym((y_max.abs().max(y_min.abs())).max(1.0));
            (-m, m)
        } else if y_min < 0.0 {
            let m = y_min.abs().max(1.0) + pad_sym(y_min.abs().max(1.0));
            (-m, m)
        } else {
            let top = if y_max <= 0.0 { 1.0 } else { y_max };
            (0.0, top + pad_pos(top.max(0.1)))
        };

        [(x0, x1), (y0, y1)]
    }

    fn determine_points(sigs: &[SigElement]) -> Box<[(f64, f64)]> {
        if sigs.is_empty() {
            return Default::default();
        }

        let mut points = Vec::with_capacity(sigs.len() * 3);
        points.push((sigs[0].ti(), sigs[0].lvl()));
        points.push((sigs[0].tf(), sigs[0].lvl()));
        for win in sigs.windows(2) {
            let a = win[0];
            let b = win[1];

            if (b.lvl() - a.lvl()).abs() > f64::EPSILON {
                points.push((a.tf(), b.lvl()));
            }
            points.push((b.tf(), b.lvl()));
        }

        points.into()
    }
}

impl ActiveWidget for Visualizer {
    fn render_ref(&self, ctx: &Ctx<'_>, frame: &mut Frame<'_>, area: Rect) {
        let (x_axis, y_axis, [_, (y0, y1)]) = self.create_axis();
        let total = self.points.len();
        let n_vis = Self::MAX_POINTS_TO_RENDER.min(total.max(1));
        let start = self.start_pos.min(total.saturating_sub(n_vis));
        let start_inclusive = start.saturating_sub(1);
        let end_exclusive = (start + n_vis).min(total);
        let slice = &self.points[start_inclusive..end_exclusive];

        let zero_line = if y0 < 0.0 && y1 > 0.0 {
            slice.iter().map(|(x, _)| (*x, 0.0)).collect::<Box<[_]>>()
        } else {
            Default::default()
        };

        let zero_guide = Dataset::default()
            .graph_type(GraphType::Line)
            .marker(symbols::Marker::Dot)
            .style(Theme::TEXT)
            .data(&zero_line);

        let waveform = Dataset::default()
            .graph_type(GraphType::Line)
            .marker(symbols::Marker::HalfBlock)
            .style(Theme::WAVEFORM)
            .data(slice);

        let block = Block::bordered()
            .style(Theme::BORDER_PRIMARY)
            .border_type(if ctx.mode.is_visualizer() {
                BorderType::Double
            } else {
                BorderType::Rounded
            })
            .title_top(
                Line::from_iter([
                    Span::raw("[ ").style(Theme::BORDER_PRIMARY),
                    Span::raw("Waveform").style(Theme::WARN),
                    Span::raw(" ]").style(Theme::BORDER_PRIMARY),
                ])
                .alignment(Alignment::Center),
            );

        frame.render_widget(
            Chart::new(vec![zero_guide, waveform])
                .x_axis(x_axis)
                .y_axis(y_axis)
                .block(block),
            area,
        );
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if let KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        } = key
        {
            match code {
                KeyCode::Left => self.left(),
                KeyCode::Right => self.right(),
                _ => {}
            }
        };
    }

    fn tick(&mut self, _: Instant) {
        let Ok(sigs) = self.sig_rx.try_recv() else {
            return;
        };

        self.sig_elems = sigs;
        self.points = Self::determine_points(&self.sig_elems);

        let total = self.points.len();
        let n_vis = Self::MAX_POINTS_TO_RENDER.min(total.max(1));
        let max_start = total.saturating_sub(n_vis);
        self.start_pos = max_start;
    }
}
