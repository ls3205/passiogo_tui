use ratatui::crossterm::event::{self, Event, KeyEvent};

use crate::AppState;

pub fn handler(state: &mut AppState) -> bool {
    if let Ok(Event::Key(k)) = event::read() {
        handle_sys_list(k, state);
        return handle_default(k, state);
    }

    false
}

pub fn handle_default(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Backspace => {
            state.sys_state.system_id = None;
        }
        event::KeyCode::Esc => return true,
        _ => {}
    }

    false
}

pub fn handle_sys_list(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Char(c) => match c {
            'j' => {
                state.sys_state.systems_list_state.select_next();
            }
            'k' => {
                state.sys_state.systems_list_state.select_previous();
            }
            'G' => {
                state.sys_state.systems_list_state.select_last();
            }
            'g' => {
                state.sys_state.systems_list_state.select_first();
            }
            _ => {}
        },
        event::KeyCode::Enter => {
            if let Some(sys) = state.sys_state.systems_list_state.selected() {
                if let Some(sys_item) = state.sys_state.systems_vec.get(sys) {
                    state.sys_state.system_id = Some(u16::try_from(sys_item.id).unwrap_or(0));
                }
            }
        }
        _ => {}
    }
    false
}
