use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

mod keys;
mod ui;

#[derive(Default, Debug)]
pub struct AppState {}

fn main() -> Result<()> {
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
