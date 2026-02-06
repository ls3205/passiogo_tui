use ratatui_recipe::Pages;

mod home;

#[derive(Pages)]
pub enum AppPages {
    Home(home::HomeScreen),
}

impl Default for AppPages {
    fn default() -> Self {
        AppPages::Home(home::HomeScreen::default())
    }
}
