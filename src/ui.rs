use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Widget,
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
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

    if state.sys_state.system_id.is_none() {
        render_list(frame, state);
    }
}

fn render_list(frame: &mut Frame, state: &mut AppState) {
    if state.sys_state.loading {
        let [area] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
        let text_area = area.centered(Constraint::Percentage(25), Constraint::Percentage(25));

        Paragraph::new(" Loading Systems ")
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(text_area, frame.buffer_mut());
    } else if state.sys_state.systems_vec.is_empty() {
        let [area] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
        let text_area = area.centered(Constraint::Percentage(25), Constraint::Percentage(25));

        Paragraph::new(" No Systems Found ")
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(text_area, frame.buffer_mut());
    } else {
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(3)
            .areas(frame.area());

        let list =
            List::new(state.sys_state.systems_vec.iter().map(|s| {
                ListItem::from(s.name.clone().unwrap_or(s.id.to_string())).fg(Color::Green)
            }))
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Green));

        frame.render_stateful_widget(list, inner_area, &mut state.sys_state.systems_list_state);
    }
}
