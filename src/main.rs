use color_eyre::eyre::Result;
use passiogo_rs::PassioGoClient;
use ratatui::{DefaultTerminal, widgets::ListState};
use std::sync::Arc;

mod keys;
mod ui;
mod views;

#[derive(Default, Debug, Clone)]
pub struct AppState {
    client: Arc<PassioGoClient>,
    sys_state: SysState,
}

#[derive(Default, Debug, Clone)]
struct SysState {
    loading: bool,
    system_id: Option<u16>,
    systems_vec: Vec<passiogo_rs::TransportationSystemData>,
    systems_list_state: ListState,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut state = AppState::default();
    let term = ratatui::init();

    let res = run(term, &mut state).await;

    ratatui::restore();

    res
}

async fn run(mut term: DefaultTerminal, state: &mut AppState) -> Result<()> {
    if state.sys_state.system_id.is_none() {
        let _ = views::sys_list(term, state).await;
    }

    Ok(())
}
