use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    prelude::Widget,
    style::{Color, Style, Stylize},
    text::{Span, ToSpan},
    widgets::{Block, Padding, Paragraph, Row, Table},
};

use crate::AppState;

pub fn render(frame: &mut Frame, state: &mut AppState) {
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
}
