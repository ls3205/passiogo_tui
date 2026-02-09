use chrono::{DateTime, Local};
use passiogo_rs::{ETAData, RouteData, StopData, SystemAlertData, VehicleData};
use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::Color,
    text::ToSpan,
    widgets::{Block, Widget},
};
use ratatui_recipe::StatefulPage;

use crate::{GlobalState, pages::pageID, utils::keybindinator};

#[derive(Default)]
pub struct MapScreen {
    routes: Vec<RouteData>,
    stops: Vec<StopData>,
    buses: Vec<VehicleData>,
    alerts: Vec<SystemAlertData>,
    etas: Vec<ETAData>,
    last_fetched: DateTime<Local>,
}

impl StatefulPage<pageID, GlobalState> for MapScreen {
    fn draw(&mut self, frame: &mut ratatui::Frame, state: &GlobalState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Min(3)])
            .split(frame.area());

        let header_text = format!(
            " PassioGo - {} ",
            state
                .system
                .clone()
                .unwrap_or_default()
                .name
                .unwrap_or("<Unamed>".to_string())
        );
        let header = header_text.to_span().into_centered_line();

        let footer = keybindinator(
            vec![
                (String::from("Back"), String::from("[Backspace]")),
                (String::from("Quit"), String::from("[Esc]")),
            ],
            Color::Green,
            Color::Yellow,
        )
        .alignment(ratatui::layout::HorizontalAlignment::Center);

        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title_top(header)
            .title_bottom(footer)
            .render(chunks[0], frame.buffer_mut());
    }

    async fn on_enter(&mut self, router: ratatui_recipe::Router<pageID>, state: &mut GlobalState) {
        if state.system.is_none() {
            router.back();
        }
    }

    async fn on_event(
        &mut self,
        event: ratatui::crossterm::event::Event,
        router: ratatui_recipe::Router<pageID>,
        state: &mut GlobalState,
    ) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Backspace => {
                    router.back();
                    state.system = None;
                }
                KeyCode::Esc => router.exit(),
                _ => {}
            }
        }
    }
}
