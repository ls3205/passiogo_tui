use chrono::{DateTime, Local};
use passiogo_rs::{ETAData, RouteData, StopData, SystemAlertData, VehicleData};
use ratatui_recipe::StatefulPage;

use crate::{GlobalState, pages::pageID};

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
    fn draw(&mut self, frame: &mut ratatui::Frame, state: &GlobalState) {}

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
    }
}
