use ratatui::crossterm::event::{self, Event, KeyEvent};

use crate::AppState;

pub fn handler(state: &mut AppState) -> bool {
    if let Ok(Event::Key(k)) = event::read() {
        return handle_default(k, state);
    }

    false
}

pub fn handle_default(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => return true,
        _ => {}
    }

    false
}
