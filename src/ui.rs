use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Widget,
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

use crate::AppState;

pub fn render(
    frame: &mut Frame,
    state: &mut AppState,
    fnc: fn(frame: &mut Frame, state: &mut AppState),
) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(
            " passiogo tui "
                .to_span()
                .into_centered_line()
                .fg(Color::White),
        )
        .fg(Color::default())
        .render(border_area, frame.buffer_mut());

    fnc(frame, state);
}
