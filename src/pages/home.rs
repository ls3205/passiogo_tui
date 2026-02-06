use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, ToSpan},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use ratatui_recipe::{Page, Router};

use crate::pages::pageID;

use passiogo_rs::{PassioGoClient, TransportationSystemData};

#[derive(Default)]
pub struct HomeScreen {
    systems: Vec<TransportationSystemData>,
    list_state: ListState,
    loading: bool,
    search_mode: bool,
    search_input: String,
}

impl HomeScreen {
    fn filtered(&self) -> Vec<&TransportationSystemData> {
        if self.search_input.is_empty() {
            self.systems.iter().collect()
        } else {
            let needle = self.search_input.to_lowercase();
            self.systems
                .iter()
                .filter(|s| {
                    s.name
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&needle)
                })
                .collect()
        }
    }
}

impl Page<pageID> for HomeScreen {
    fn draw(&mut self, frame: &mut Frame) {
        // choose layout depending on whether the search input is visible
        let show_bottom = self.search_mode || !self.search_input.is_empty();

        if show_bottom {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(3),
                    Constraint::Length(3),
                ])
                .split(frame.size());

            let header_text = if self.loading {
                "PassioGo - Systems (Loading...)"
            } else {
                "PassioGo - Systems"
            };
            let header = Paragraph::new(vec![
                Line::from(header_text),
                Line::from(""),
                Line::from("Use j/k or ↑/↓ to navigate, / to search, Enter to select, Esc to exit"),
            ])
            .block(Block::default().borders(Borders::ALL));

            frame.render_widget(header, chunks[0]);

            if self.loading {
                let loading = Paragraph::new(vec![Line::from("Loading systems...")])
                    .block(Block::default().borders(Borders::ALL).title("Systems"));
                frame.render_widget(loading, chunks[1]);
                return;
            }

            // render list and bottom in the branch below
            // create filtered and items
            let filtered = self.filtered();

            if filtered.is_empty() {
                let empty = Paragraph::new(vec![Line::from("No systems match the filter.")])
                    .block(Block::default().borders(Borders::ALL).title("Systems"));
                frame.render_widget(empty, chunks[1]);
            } else {
                let items: Vec<ListItem> = filtered
                    .into_iter()
                    .map(|s| {
                        let name = s.name.as_deref().unwrap_or("<unnamed>");
                        let line = Line::from(format!("{} - {}", s.id, name));
                        ListItem::new(vec![line])
                    })
                    .collect();

                // Ensure selection valid for filtered list
                if self.list_state.selected().is_none() {
                    self.list_state.select(Some(0));
                } else if let Some(idx) = self.list_state.selected() {
                    if idx >= items.len() {
                        self.list_state.select(Some(0));
                    }
                }

                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Systems")
                            .title_bottom(
                                "j/k ↑/↓: move  /: search  Enter: select  Esc: exit"
                                    .to_span()
                                    .into_centered_line(),
                            ),
                    )
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">>");

                frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
            }

            // bottom area: show search input
            let mut bottom_lines = vec![];
            if self.search_mode {
                bottom_lines.push(Line::from(format!("Search: /{}_", self.search_input)));
            } else {
                bottom_lines.push(Line::from(format!("Search: /{}", self.search_input)));
            }

            let bottom = Paragraph::new(bottom_lines)
                .block(Block::default().borders(Borders::ALL).title("Search"));
            frame.render_widget(bottom, chunks[2]);

            return;
        }

        // default layout without bottom input
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(3)])
            .split(frame.size());

        let header_text = if self.loading {
            "PassioGo - Systems (Loading...)"
        } else {
            "PassioGo - Systems"
        };
        let header = Paragraph::new(vec![
            Line::from(header_text),
            Line::from(""),
            Line::from("Use j/k or ↑/↓ to navigate, / to search, Enter to select, Esc to exit"),
        ])
        .block(Block::default().borders(Borders::ALL));

        frame.render_widget(header, chunks[0]);

        if self.loading {
            let loading = Paragraph::new(vec![Line::from("Loading systems...")])
                .block(Block::default().borders(Borders::ALL).title("Systems"));
            frame.render_widget(loading, chunks[1]);
            return;
        }

        let filtered = self.filtered();

        if filtered.is_empty() {
            let empty = Paragraph::new(vec![Line::from("No systems match the filter.")])
                .block(Block::default().borders(Borders::ALL).title("Systems"));
            frame.render_widget(empty, chunks[1]);
        } else {
            let items: Vec<ListItem> = filtered
                .into_iter()
                .map(|s| {
                    let name = s.name.as_deref().unwrap_or("<unnamed>");
                    let line = Line::from(format!("{} - {}", s.id, name));
                    ListItem::new(vec![line])
                })
                .collect();

            // Ensure selection valid for filtered list
            if self.list_state.selected().is_none() {
                self.list_state.select(Some(0));
            } else if let Some(idx) = self.list_state.selected() {
                if idx >= items.len() {
                    self.list_state.select(Some(0));
                }
            }

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Systems")
                        .title_bottom(
                            "j/k ↑/↓: move  /: search  Enter: select  Esc: exit"
                                .to_span()
                                .into_centered_line(),
                        ),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">>");

            frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
        }
        let filtered = self.filtered();

        if filtered.is_empty() {
            let empty = Paragraph::new(vec![Line::from("No systems match the filter.")])
                .block(Block::default().borders(Borders::ALL).title("Systems"));
            frame.render_widget(empty, chunks[1]);
        } else {
            let items: Vec<ListItem> = filtered
                .into_iter()
                .map(|s| {
                    let name = s.name.as_deref().unwrap_or("<unnamed>");
                    let line = Line::from(format!("{} - {}", s.id, name));
                    ListItem::new(vec![line])
                })
                .collect();

            // Ensure selection valid for filtered list
            if self.list_state.selected().is_none() {
                self.list_state.select(Some(0));
            } else if let Some(idx) = self.list_state.selected() {
                if idx >= items.len() {
                    self.list_state.select(Some(0));
                }
            }

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Systems")
                        .title_bottom("j/k ↑/↓: move  /: search  Enter: select  Esc: exit"),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">>");

            frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
        }

        // bottom area: show search input only when entering search mode or when a search query exists
        if self.search_mode || !self.search_input.is_empty() {
            let mut bottom_lines = vec![];
            if self.search_mode {
                bottom_lines.push(Line::from(format!("Search: /{}_", self.search_input)));
            } else {
                bottom_lines.push(Line::from(format!("Search: /{}", self.search_input)));
            }

            let bottom = Paragraph::new(bottom_lines)
                .block(Block::default().borders(Borders::ALL).title("Search"));
            frame.render_widget(bottom, chunks[2]);
        }
    }

    async fn on_enter(&mut self, router: Router<pageID>) {
        self.loading = true;
        router.redraw();

        let client = PassioGoClient::default();
        let systems = client.get_systems().await.unwrap_or_default();
        self.systems = systems;
        self.loading = false;

        // Fix up selection
        if self.systems.is_empty() {
            self.list_state.select(None);
        } else if self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }

        router.redraw();
    }

    async fn on_event(&mut self, event: Event, router: Router<pageID>) {
        if let Event::Key(key_event) = event {
            // If currently editing search input
            if self.search_mode {
                match key_event.code {
                    KeyCode::Char(c) => {
                        self.search_input.push(c);
                    }
                    KeyCode::Backspace => {
                        self.search_input.pop();
                    }
                    KeyCode::Enter => {
                        // If enter on empty input, remove input field entirely
                        if self.search_input.is_empty() {
                            self.search_mode = false;
                            // keep search_input empty
                        } else {
                            // stop editing but keep the input visually
                            self.search_mode = false;
                            // ensure selection is valid in filtered results
                            let filtered_len = self.filtered().len();
                            if filtered_len == 0 {
                                self.list_state.select(None);
                            } else {
                                self.list_state.select(Some(0));
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Close search mode and clear input instead of exiting
                        self.search_mode = false;
                        self.search_input.clear();
                    }
                    _ => {}
                }

                router.redraw();
                return;
            }

            match key_event.code {
                KeyCode::Char('/') => {
                    // enter search mode
                    self.search_mode = true;
                    // leave existing search_input as-is to edit
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    let len = self.filtered().len();
                    if len == 0 {
                        router.redraw();
                        return;
                    }
                    let next = match self.list_state.selected() {
                        Some(i) if i + 1 < len => Some(i + 1),
                        Some(_) => Some(0),
                        None => Some(0),
                    };
                    self.list_state.select(next);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    let len = self.filtered().len();
                    if len == 0 {
                        router.redraw();
                        return;
                    }
                    let prev = match self.list_state.selected() {
                        Some(i) if i > 0 => Some(i - 1),
                        Some(_) => Some(len - 1),
                        None => Some(0),
                    };
                    self.list_state.select(prev);
                }
                KeyCode::Enter => {
                    // Future: open details for the selected system.
                }
                KeyCode::Esc => {
                    if self.search_input.is_empty() {
                        router.exit();
                    } else {
                        self.search_input.clear();
                    }
                }
                _ => {}
            }

            router.redraw();
        }
    }
}
