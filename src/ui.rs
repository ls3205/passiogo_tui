use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Widget,
    style::{Color, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Paragraph},
};
use tokio::sync::oneshot;

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
    println!("Render list");
    if state.sys_state.systems_vec.is_empty() && !state.sys_state.loading {
        println!("Fetch list");
        state.sys_state.loading = true;

        let (tx, mut rx) = tokio::sync::oneshot::channel();
        let client = state.client.clone();
        let handle = tokio::runtime::Handle::current();
        tokio::task::spawn_blocking(move || {
            handle.block_on(async {
                let data = client.get_systems().await.unwrap_or_default();
                let _ = tx.send(data);
            });
        });

        // let data = rx.try_recv();
        let data = rx.blocking_recv();
        println!("{:#?}", data);

        state.sys_state.loading = false;
        println!("End fetch");
    }

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
            }));

        frame.render_stateful_widget(list, inner_area, &mut state.sys_state.systems_list_state);
    }
}
