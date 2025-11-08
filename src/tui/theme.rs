#![cfg(feature = "tui")]

use ratatui::style::{Color, Style};

#[derive(Clone, Debug)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub border: Color,
    pub accent: Color,
    pub list_highlight_bg: Color,
    pub list_highlight_fg: Color,
    pub sel_highlight_bg: Color,
    pub sel_highlight_fg: Color,
    pub ok: Color,
    pub error: Color,
}

impl Theme {
    pub fn gitui_dark() -> Self {
        // A simple palette inspired by GitUI
        Self {
            bg: Color::Rgb(12, 12, 12),
            fg: Color::Rgb(201, 209, 217),
            border: Color::Rgb(48, 54, 61),
            accent: Color::Rgb(88, 166, 255),
            list_highlight_bg: Color::Rgb(31, 111, 235),
            list_highlight_fg: Color::Rgb(255, 255, 255),
            sel_highlight_bg: Color::Rgb(34, 197, 94),
            sel_highlight_fg: Color::Rgb(0, 0, 0),
            ok: Color::Rgb(46, 160, 67),
            error: Color::Rgb(248, 81, 73),
        }
    }

    pub fn light() -> Self {
        Self {
            bg: Color::Rgb(250, 250, 250),
            fg: Color::Rgb(30, 30, 30),
            border: Color::Rgb(200, 200, 200),
            accent: Color::Rgb(25, 118, 210),
            list_highlight_bg: Color::Rgb(187, 222, 251),
            list_highlight_fg: Color::Rgb(0, 0, 0),
            sel_highlight_bg: Color::Rgb(200, 230, 201),
            sel_highlight_fg: Color::Rgb(0, 0, 0),
            ok: Color::Rgb(46, 160, 67),
            error: Color::Rgb(211, 47, 47),
        }
    }
}

pub fn resolve(name: Option<String>) -> Theme {
    match name.as_deref() {
        Some("light") => Theme::light(),
        _ => Theme::gitui_dark(),
    }
}

