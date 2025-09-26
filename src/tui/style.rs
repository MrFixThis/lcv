use ratatui::style::{Color, Modifier, Style};

macro_rules! theme {
    ($( $name:ident = $val:expr ),* $(,)?) => {
        pub struct Theme;
        impl Theme {
            $(
                pub const $name: ::ratatui::style::Style = $val;
            )*
        }
    };
}

theme! {
    TITLE            = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
    SUB_TITLE        = Style::new().fg(Color::LightMagenta).add_modifier(Modifier::BOLD),
    TEXT             = Style::new().fg(Color::White),
    HINT             = Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD),
    ERROR            = Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
    WARN             = Style::new().fg(Color::LightYellow).add_modifier(Modifier::BOLD),
    BORDER_PRIMARY   = Style::new().fg(Color::Green),
    BORDER_SECONDARY = Style::new().fg(Color::LightGreen),
    BORDER_TERNARY   = Style::new().fg(Color::Yellow),
    WAVEFORM         = Style::new().fg(Color::LightCyan),
    HIGHLIGHT_ITEM   = Style::new().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD),
}
