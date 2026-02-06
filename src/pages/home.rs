use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    text::Line,
    widgets::Paragraph,
};
use ratatui_recipe::{Page, Router};

use crate::pages::pageID;

#[derive(Default)]
pub struct HomeScreen {
    counter: u32,
}

impl Page<pageID> for HomeScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let text = Paragraph::new(vec![
            Line::from("Hello ratapp!"),
            Line::from(""),
            Line::from("This is the home screen. Welcome!"),
            Line::from(""),
            Line::from(format!("Counter: {}", self.counter)),
            Line::from(""),
            Line::from("Use the arrows up and down to update the counter."),
        ]);

        frame.render_widget(text, frame.area());
    }

    async fn on_event(&mut self, event: Event, router: Router<pageID>) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => {
                    router.exit();
                }
                _ => {}
            }

            router.redraw(); // Add this line to trigger a re-draw after handling the event.
        }
    }
}
