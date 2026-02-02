use color_eyre::eyre::Result;
use passiogo_rs::PassioGoClient;
use ratatui::{DefaultTerminal, widgets::ListState};
use std::sync::Arc;

mod keys;
mod req;
mod ui;

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

    let res = run(term, &mut state);

    ratatui::restore();

    res
}

fn run(mut term: DefaultTerminal, state: &mut AppState) -> Result<()> {
    loop {
        // rendering
        term.draw(|f| ui::render(f, state))?;
        // input handling
        if keys::handler(state) {
            break;
        };
    }
    Ok(())
}
