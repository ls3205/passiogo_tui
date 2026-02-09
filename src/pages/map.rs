use chrono::{DateTime, Local};
use passiogo_rs::{ETAData, RouteData, StopData, SystemAlertData, VehicleData};
use ratatui::{
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::ToSpan,
    widgets::{Block, Row, Table, Widget},
};
use ratatui_recipe::StatefulPage;

use crate::{GlobalState, pages::pageID, utils::keybindinator};

#[derive(Default)]
pub struct MapScreen {
    system_id: i64,
    routes: Vec<RouteData>,
    stops: Vec<StopData>,
    buses: Vec<VehicleData>,
    alerts: Vec<SystemAlertData>,
    etas: Vec<ETAData>,
    last_fetched: DateTime<Local>,

    stop_matrix: Vec<Vec<StopData>>,
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

        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title_top(header)
            .title_bottom(footer);

        // render the outer block and compute its inner area for the table
        block.clone().render(chunks[0], frame.buffer_mut());
        let inner = block.inner(chunks[0]);

        // render stops as independent elements positioned by latitude/longitude within the block inner area
        let valid_stops: Vec<&StopData> = self
            .stops
            .iter()
            .filter(|s| s.latitude.is_some() && s.longitude.is_some())
            .collect();

        if !valid_stops.is_empty() {
            // compute geographic bounds
            let mut min_lat = f64::INFINITY;
            let mut max_lat = f64::NEG_INFINITY;
            let mut min_lon = f64::INFINITY;
            let mut max_lon = f64::NEG_INFINITY;
            for s in &valid_stops {
                let lat = s.latitude.unwrap();
                let lon = s.longitude.unwrap();
                if lat < min_lat {
                    min_lat = lat
                }
                if lat > max_lat {
                    max_lat = lat
                }
                if lon < min_lon {
                    min_lon = lon
                }
                if lon > max_lon {
                    max_lon = lon
                }
            }
            let lat_span = if (max_lat - min_lat).abs() < f64::EPSILON {
                1.0
            } else {
                max_lat - min_lat
            };
            let lon_span = if (max_lon - min_lon).abs() < f64::EPSILON {
                1.0
            } else {
                max_lon - min_lon
            };

            let buf = frame.buffer_mut();

            let inner_x = inner.x.saturating_add(1);
            let inner_y = inner.y.saturating_add(1);
            let inner_w = inner.width.saturating_sub(2).max(1);
            let inner_h = inner.height.saturating_sub(2).max(1);

            for stop in valid_stops {
                let lat = stop.latitude.unwrap();
                let lon = stop.longitude.unwrap();

                let x_rel = ((lon - min_lon) / lon_span) * ((inner_w.saturating_sub(1)) as f64);
                let y_rel = ((max_lat - lat) / lat_span) * ((inner_h.saturating_sub(1)) as f64);

                let x = inner_x.saturating_add(x_rel.round() as u16);
                let y = inner_y.saturating_add(y_rel.round() as u16);

                let label = stop.name.clone().unwrap_or_default();
                let display = if label.is_empty() {
                    "•".to_string()
                } else {
                    label
                };

                for (i, ch) in display.chars().enumerate() {
                    let px = x.saturating_add(i as u16);
                    if px >= inner_x.saturating_add(inner_w) {
                        break;
                    }
                    if y >= inner_y.saturating_add(inner_h) {
                        break;
                    }
                    if let Some(cell) = buf.cell_mut((px, y)) {
                        let s = ch.to_string();
                        cell.set_symbol(&s);
                        cell.set_style(Style::default().fg(Color::Yellow));
                    }
                }
            }
        }
    }

    async fn on_enter(&mut self, router: ratatui_recipe::Router<pageID>, state: &mut GlobalState) {
        if state.system.is_none() {
            router.back();
        } else {
            self.system_id = state.system.as_ref().unwrap().id;
        }

        let stop_data = state
            .client
            .get_stops(self.system_id)
            .await
            .unwrap_or_default();

        self.stops = stop_data.clone();

        let valid_stops: Vec<StopData> = self
            .stops
            .iter()
            .filter(|&s| s.latitude.is_some() && s.longitude.is_some())
            .cloned()
            .collect();

        if valid_stops.is_empty() {
            self.stop_matrix = Vec::new();
        } else {
            // compute geographic bounds
            let mut min_lat = f64::INFINITY;
            let mut max_lat = f64::NEG_INFINITY;
            let mut min_lon = f64::INFINITY;
            let mut max_lon = f64::NEG_INFINITY;

            for s in &valid_stops {
                let lat = s.latitude.unwrap();
                let lon = s.longitude.unwrap();
                if lat < min_lat {
                    min_lat = lat
                }
                if lat > max_lat {
                    max_lat = lat
                }
                if lon < min_lon {
                    min_lon = lon
                }
                if lon > max_lon {
                    max_lon = lon
                }
            }

            let lat_span = if (max_lat - min_lat).abs() < f64::EPSILON {
                1.0
            } else {
                max_lat - min_lat
            };
            let lon_span = if (max_lon - min_lon).abs() < f64::EPSILON {
                1.0
            } else {
                max_lon - min_lon
            };

            // choose number of rows for map (height). Keep fixed to fit on screen; adjust as needed.
            let rows = 20_usize.min(valid_stops.len().max(1));
            // compute columns to preserve geographic aspect ratio (lon per lat)
            let mut cols = ((lon_span / lat_span) * rows as f64).round() as usize;
            if cols == 0 {
                cols = 1
            }

            let mut matrix_opt: Vec<Vec<Option<StopData>>> = vec![vec![None; cols]; rows];

            for stop in &valid_stops {
                let lat = stop.latitude.unwrap();
                let lon = stop.longitude.unwrap();

                // map to row/col proportionally across the grid (edges inclusive)
                let row_ratio = (max_lat - lat) / lat_span;
                let mut r = (row_ratio * (rows.saturating_sub(1) as f64)).round() as isize;
                if r < 0 {
                    r = 0
                }
                if r as usize >= rows {
                    r = (rows - 1) as isize
                }

                let col_ratio = (lon - min_lon) / lon_span;
                let mut c = (col_ratio * (cols.saturating_sub(1) as f64)).round() as isize;
                if c < 0 {
                    c = 0
                }
                if c as usize >= cols {
                    c = (cols - 1) as isize
                }

                // if target occupied find nearest empty cell (min manhattan distance)
                if matrix_opt[r as usize][c as usize].is_none() {
                    matrix_opt[r as usize][c as usize] = Some(stop.clone());
                } else {
                    let mut best: Option<(usize, usize, usize)> = None; // (r,c,dist)
                    for rr in 0..rows {
                        for cc in 0..cols {
                            if matrix_opt[rr][cc].is_none() {
                                let dist =
                                    ((rr as isize - r).abs() + (cc as isize - c).abs()) as usize;
                                if best.is_none() || dist < best.unwrap().2 {
                                    best = Some((rr, cc, dist));
                                }
                            }
                        }
                    }
                    if let Some((br, bc, _)) = best {
                        matrix_opt[br][bc] = Some(stop.clone());
                    }
                }
            }

            // placeholder is empty cell with no name
            let mut placeholder = valid_stops[0].clone();
            placeholder.name = Some(String::new());
            placeholder.latitude = None;
            placeholder.longitude = None;

            let mut matrix: Vec<Vec<StopData>> = Vec::with_capacity(rows);
            for row in matrix_opt {
                let mut rvec = Vec::with_capacity(cols);
                for cell in row {
                    rvec.push(cell.unwrap_or_else(|| placeholder.clone()));
                }
                matrix.push(rvec);
            }

            self.stop_matrix = matrix;
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
