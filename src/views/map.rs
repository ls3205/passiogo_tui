use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Widget,
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};

use crate::{AppState, keys, ui};

pub async fn map(mut term: DefaultTerminal, state: &mut AppState) -> Result<()> {
    if !state.map_state.loading && state.sys_state.system_id.is_some() {
        state.map_state.loading = true;

        state.map_state.alerts = state
            .client
            .get_alerts(state.sys_state.system_id.unwrap() as i64)
            .await
            .unwrap_or_default();

        state.map_state.routes = state
            .client
            .get_routes(state.sys_state.system_id.unwrap() as i64)
            .await
            .unwrap_or_default();

        state.map_state.stops = state
            .client
            .get_stops(state.sys_state.system_id.unwrap() as i64)
            .await
            .unwrap_or_default();

        state.map_state.buses = state
            .client
            .get_buses(state.sys_state.system_id.unwrap() as i64)
            .await
            .unwrap_or_default();

        state.map_state.last_fetched = chrono::Local::now();
        state.sys_state.loading = false;
    }

    loop {
        // reqs

        // rendering
        term.draw(|f| ui::render(f, state, render_map))?;

        // input handling
        if let Ok(Event::Key(k)) = event::read() {
            keys::handle_sys_list(k, state);
            if keys::handle_default(k, state) {
                break;
            }
        };
    }
    Ok(())
}

fn render_map(frame: &mut Frame, state: &mut AppState) {
    if state.sys_state.loading {
        let [area] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
        let text_area = area.centered(Constraint::Percentage(25), Constraint::Percentage(25));

        Paragraph::new(" Loading Map ")
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(text_area, frame.buffer_mut());
    } else if state.map_state.routes.is_empty() {
        let [area] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
        let text_area = area.centered(Constraint::Percentage(25), Constraint::Percentage(25));

        Paragraph::new(" No Routes Found ")
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(text_area, frame.buffer_mut());
    } else {
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(3)
            .areas(frame.area());

        let list =
            List::new(state.map_state.routes.iter().map(|s| {
                ListItem::from(s.name.clone().unwrap_or(s.id.to_string())).fg(Color::Green)
            }))
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Green));

        frame.render_widget(list, inner_area);
    }
}
