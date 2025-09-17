use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};

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
        todo!()
    }
}
