use crate::views::types::Views;
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
    view: Views,
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
    match state.view {
        Views::List => {
            let _ = views::list::sys_list(term, state).await;
        }
        Views::Map => {}
        _ => {}
    }

    Ok(())
}
