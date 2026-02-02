use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};

use crate::{AppState, keys, ui};

pub async fn sys_list(mut term: DefaultTerminal, state: &mut AppState) -> Result<()> {
    if state.sys_state.systems_vec.is_empty() && !state.sys_state.loading {
        state.sys_state.loading = true;
        state.sys_state.systems_vec = state.client.get_systems().await.unwrap_or_default();
        state.sys_state.loading = false;
    }

    loop {
        // reqs

        // rendering
        term.draw(|f| ui::render(f, state))?;

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
