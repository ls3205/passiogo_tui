use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, ToSpan},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use ratatui_recipe::{Router, StatefulPage};

use crate::{GlobalState, pages::pageID, utils::keybindinator};

use passiogo_rs::TransportationSystemData;

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

impl StatefulPage<pageID, GlobalState> for HomeScreen {
    fn draw(&mut self, frame: &mut Frame, _state: &GlobalState) {
        let show_bottom = self.search_mode || !self.search_input.is_empty();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(if show_bottom { 3 } else { 0 }),
            ])
            .split(frame.area());

        let header_text = if self.loading {
            " PassioGo - Systems (Loading...) "
        } else {
            " PassioGo - Systems "
        };
        let header = header_text.to_span().into_centered_line();

        let footer = keybindinator(
            vec![
                (String::from("Up"), String::from("[k]")),
                (String::from("Down"), String::from("[j]")),
                (String::from("Search"), String::from("[/]")),
                (String::from("Select"), String::from("[Enter]")),
                (
                    String::from(if self.search_mode {
                        "Exit Search"
                    } else {
                        "Quit"
                    }),
                    String::from("[Esc]"),
                ),
            ],
            Color::Green,
            Color::Yellow,
        )
        .alignment(ratatui::layout::HorizontalAlignment::Center);

        if self.loading {
            let loading = Paragraph::new(vec![Line::from("Loading systems...")]).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(header),
            );
            frame.render_widget(loading, chunks[0]);
            return;
        }

        let filtered = self.filtered();

        if filtered.is_empty() {
            let empty = Paragraph::new(vec![Line::from("No systems match the filter.")]).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(header)
                    .title_bottom(footer),
            );
            frame.render_widget(empty, chunks[0]);
        } else {
            let items: Vec<ListItem> = filtered
                .into_iter()
                .map(|s| {
                    let name = s.name.as_deref().unwrap_or("<unnamed>");
                    let line = Line::from(format!("{} - {}", s.id, name));
                    ListItem::new(vec![line])
                })
                .collect();

            if self.list_state.selected().is_none() {
                self.list_state.select(Some(0));
            } else if let Some(idx) = self.list_state.selected()
                && idx >= items.len()
            {
                self.list_state.select(Some(0));
            }

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .title(header)
                        .title_bottom(footer),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">>");

            frame.render_stateful_widget(list, chunks[0], &mut self.list_state);
        }

        let mut bottom_lines = vec![];
        if self.search_mode {
            bottom_lines.push(Line::from(format!("Search: /{}_", self.search_input)));
        } else {
            bottom_lines.push(Line::from(format!("Search: /{}", self.search_input)));
        }

        let bottom = Paragraph::new(bottom_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search")
                .fg(if self.search_mode {
                    Color::Green
                } else {
                    Color::default()
                }),
        );

        if show_bottom {
            frame.render_widget(bottom, chunks[1])
        };
    }

    async fn on_enter(&mut self, router: Router<pageID>, state: &mut GlobalState) {
        self.loading = true;
        router.redraw();

        let systems = state.client.get_systems().await.unwrap_or_default();
        self.systems = systems;
        self.loading = false;

        if self.systems.is_empty() {
            self.list_state.select(None);
        } else if self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }

        router.redraw();
    }

    async fn on_event(&mut self, event: Event, router: Router<pageID>, state: &mut GlobalState) {
        if let Event::Key(key_event) = event {
            if self.search_mode {
                match key_event.code {
                    KeyCode::Char(c) => {
                        self.search_input.push(c);
                    }
                    KeyCode::Backspace => {
                        self.search_input.pop();
                    }
                    KeyCode::Enter => {
                        if self.search_input.is_empty() {
                            self.search_mode = false;
                        } else {
                            self.search_mode = false;
                            let filtered_len = self.filtered().len();
                            if filtered_len == 0 {
                                self.list_state.select(None);
                            } else {
                                self.list_state.select(Some(0));
                            }
                        }
                    }
                    KeyCode::Esc => {
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
                    self.search_mode = true;
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
                    if let Some(idx) = self.list_state.selected()
                        && let Some(sys) = self.systems.get(idx)
                    {
                        state.system_id = Some(sys.id);
                        println!("{:#?}", state.system_id);
                    }
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
