use sdl2::{render::{Canvas, TextureCreator}, video::{Window, WindowContext}, pixels::Color, ttf::Font, event::Event, keyboard::Keycode};
use crate::{ app::{App, AppState, GameState}, game_object::GameObject, input::button_module::{Button, TextAlign}, UI::text::Label};

enum MenuSelector {
    Play,
    Settings,
    Exit
}

pub struct GameLogic<'a> { // here we define the data we use on our script
    opt_list: [&'a MenuSelector; 3],
    actual_opt: &'a MenuSelector,
    actual_setting: usize,
    btn_list: [Button;3],
}

impl GameLogic<'_> {
    // this is called once
    pub fn new(app: &mut App) -> Self {
        let opt_list = [&MenuSelector::Play, &MenuSelector::Settings, &MenuSelector::Exit];

        // main menu
        let play = Button::new(
            GameObject {
                active: true, x:((app.width/2) - (100/2)) as f32, y: 100.0, width: 100.0, height: 50.0},
            Some(String::from("Play")),
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None,
            TextAlign::Center
        );
        let settings = Button::new(
            GameObject {
                active: true, x:((app.width/2) - (100/2)) as f32, y: 160.0, width: 100.0, height: 50.0},
            Some(String::from("Settings")),
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None, 
            TextAlign::Center
        );
        let exit = Button::new(
            GameObject {
                active: true, x:((app.width/2) - (100/2)) as f32, y: 220.0, width: 100.0, height: 50.0},
            Some(String::from("Exit")),
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None,
            TextAlign::Center
        );

        let btn_list = [play, settings, exit];

        Self {
            opt_list,
            actual_opt: opt_list[0],
            actual_setting: 0,
            btn_list,
        }
    }

    pub fn update(&mut self, _font: &Font, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        let mut texture_creator = app.canvas.texture_creator();

        // rendering buttons
        for btn in 0..self.btn_list.len() {
            self.btn_list[btn].render(&mut app.canvas, &texture_creator, _font);
        }

        // input reading and sending stuff to the canvas
        Self::event_handler(app_state, event_pump, &mut self.actual_setting, self.actual_opt, &mut self.btn_list);
    }

    fn event_handler(app_state: &mut AppState, event_pump: &mut sdl2::EventPump, actual_setting: &mut usize, actual_opt: &MenuSelector, btn_list: &mut [Button;3]) {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => {
                    app_state.is_running = false;
                },
                _ => {}
            }

            // change system of selecting options with arrows to on clicks
                if btn_list[0].on_click(&event) {
                    app_state.state = GameState::SelectingSong
                }
                if btn_list[1].on_click(&event) {
                    app_state.state = GameState::Settings
                }
                if btn_list[2].on_click(&event) {
                    app_state.state = GameState::Quitting
                }
        }
    }
}

