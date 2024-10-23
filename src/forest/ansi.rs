use clap::builder::styling::{AnsiColor, Color, Reset, Style};

const RESET: Reset = Reset;
const TASK_NAME: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));
const TREE_NAME: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Magenta)));
const TIME: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
const DATE: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue)));
const UID: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow)));
const BOX: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)));

pub enum ForestFormat {
    TaskName,
    TreeName,
    Date,
    Time,
    Uid,
    Box,
}

pub fn format(s: &String, fmt: ForestFormat) -> String {
    match fmt {
        ForestFormat::TaskName => {
            format!("{TASK_NAME}{s}{RESET}")
        }
        ForestFormat::TreeName => {
            format!("{TREE_NAME}{s}{RESET}")
        }
        ForestFormat::Date => {
            format!("{DATE}{s}{RESET}")
        }
        ForestFormat::Time => {
            format!("{TIME}{s}{RESET}")
        }
        ForestFormat::Uid => {
            format!("{UID}{s}{RESET}")
        }
        ForestFormat::Box => {
            format!("{BOX}{s}{RESET}")
        }
    }
}
