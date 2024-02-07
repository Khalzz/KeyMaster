use app::App;

mod app;
mod key;
mod game_object;
mod load_song;

mod UI {
    pub mod text;
}

mod input {
    pub mod button_module;
    pub mod keybutton;
    pub mod slider_module;
}

mod gameplay {
    pub mod play;
    pub mod editor;
    pub mod main_menu;
    pub mod song_selector;
    pub mod calibration;
    pub mod settings;
    pub mod controller;
    pub mod alert;
}

fn main() -> Result<(), String> {
    let app = App::new(1920, 1080, "Arrowner");
    app.render();
    Ok(())
}