use app::App;

mod app;
mod key;
mod game_object;
mod load_song;

mod ui {
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
    pub mod manual_calibration;
    pub mod game_calibration;
}

fn main() -> Result<(), String> {
    let app = App::new("Arrowner");
    app.render();
    Ok(())
}