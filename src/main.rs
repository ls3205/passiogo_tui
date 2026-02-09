use passiogo_rs::{PassioGoClient, TransportationSystemData};
use ratatui_recipe::App;

use crate::pages::AppPages;

mod pages;
mod utils;

#[derive(Default, Debug)]
pub struct GlobalState {
    client: PassioGoClient,
    system: Option<TransportationSystemData>,
}

#[tokio::main]
async fn main() {
    let state = GlobalState::default();
    let mut app = App::stateful(state);

    app.run::<AppPages>().await.unwrap();
}
