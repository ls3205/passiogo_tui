use ratatui_recipe::App;

use crate::pages::AppPages;

mod pages;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.run::<AppPages>().await.unwrap();
}
