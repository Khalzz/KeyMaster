use std::{fs, ffi::OsString};

use sdl2::{render::{Canvas, TextureCreator}, video::{Window, WindowContext}, pixels::Color, ttf::Font, event::Event, keyboard::Keycode};
use crate::{ app::{App, AppState, GameState, self}, game_object::GameObject, input::button_module::Button};

enum MenuSelector {
    Play,
    Settings,
    Exit
}

pub struct GameLogic<> { // here we define the data we use on our script
    btn_list: Vec<Button>,
    loading_text: Button,
}

impl GameLogic<> {
    // this is called once
    pub fn new(mut app: &mut App) -> Self {

        // read every file in ./song
        let songs = Self::load_songs("./songs".to_owned(), &mut app);
        let loading_text = Button::new(
            GameObject {
                active: true,
                x: ((app.width / 2) - ((app.width - 20) / 2)) as f32,
                y: (app.height as f32 - 60.0),
                width: (app.width as f32 - 20.0),
                height: 50.0,
            },
            Some("Loading".to_owned()),
            Color::RGB(28, 29, 37),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None
        );

        Self {
            btn_list: songs,
            loading_text,
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        for btn in 0..self.btn_list.len() {
            self.btn_list[btn].render(&mut app.canvas, &app.texture_creator, _font);
        }

        Self::event_handler(self,app_state, event_pump,  _font, app);
    }

    fn event_handler(&mut self, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, _font: &Font, app: &mut App) {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    app_state.state = GameState::MainMenu;
                },
                _ => {}
            }
            for btn in 0..self.btn_list.len() {
                if self.btn_list[btn].on_click(&event) || self.btn_list[btn].on_lclick(&event) {
                    if btn == 0 {
                        app_state.state = GameState::MainMenu;
                    } else {
                        // add here a reset for the play
                        self.loading(&mut app.texture_creator, _font, &mut app.canvas);
                        match &self.btn_list[btn].text {
                            Some(_text) => {
                                app_state.song_folder = Some(_text.clone());

                            },
                            None => {},
                        }

                        if self.btn_list[btn].on_lclick(&event) {
                            app.reseted = false;
                            app_state.state = GameState::Editing;
                        } else {
                            app.reseted = false;
                            app_state.state = GameState::Playing;
                        }
                    }
                }
            }
        }
    }

    // this will show the loading message
    fn loading (&mut self, texture_creator: &mut TextureCreator<WindowContext>, _font: &Font, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        self.loading_text.render(canvas, texture_creator, _font);
        canvas.present();
    }

    fn load_songs(songs_folder: String, app: &mut App) -> Vec<Button>{
        let folders = fs::read_dir(&songs_folder).unwrap();
        let mut songs_buttons: Vec<Button> = vec![];
        let mut position = 50.0;

        let back_button = Button::new(GameObject {active: true, x: 10.0 as f32, y: 10.0, width: 70.0, height: 30.0},Some(String::from("Back")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None);
        songs_buttons.push(back_button);

        for entry in folders {
            if let entry = entry.unwrap() { // change this
                if entry.file_type().unwrap().is_dir() {
                    songs_buttons.push(Button::new(
                        GameObject {
                            active: true,
                            x: ((app.width / 2) - (300 / 2)) as f32,
                            y: position,
                            width: 350.0,
                            height: 50.0,
                        },
                        Some(entry.file_name().to_string_lossy().to_string()),
                        Color::RGB(100, 100, 100),
                        Color::WHITE,
                        Color::RGB(0, 200, 0),
                        Color::RGB(0, 0, 0),
                        None
                    ));
                    position += 60.0;
                }
            }
        }

        return songs_buttons
    }
}

