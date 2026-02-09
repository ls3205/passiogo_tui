use ratatui_recipe::Pages;

mod home;
mod map;

#[derive(Pages)]
pub enum AppPages {
    Home(home::HomeScreen),
    Map(map::MapScreen),
}

impl Default for AppPages {
    fn default() -> Self {
        AppPages::Home(home::HomeScreen::default())
    }
}
