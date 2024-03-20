use sdl2::{event::Event, keyboard::Keycode, pixels::Color, ttf::Font};
use crate::{ app::{App, AppState, GameState}, game_object::GameObject, input::button_module::{Button, TextAlign}};

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
        let texture_creator = app.canvas.texture_creator();

        // rendering buttons
        for btn in 0..self.btn_list.len() {
            self.btn_list[btn].render(&mut app.canvas, &texture_creator, _font);
        }

        for (i, btn) in self.btn_list.iter_mut().enumerate() {
            if i == self.actual_setting {
                btn.color = Color::RGB(0, 200, 0);
            } else {
                btn.color = Color::RGB(100, 100, 100);
            }
        }
        

        // input reading and sending stuff to the canvas
        self.event_handler(app_state, event_pump, app);
    }

    fn event_handler(&mut self, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[0]).unwrap() => {
                    // notink
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[1]).unwrap() => {
                    if self.actual_setting > usize::MIN {
                        self.actual_setting -= 1;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[2]).unwrap() => {
                    if self.actual_setting < self.btn_list.len() - 1 {
                        self.actual_setting += 1;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[3]).unwrap() => {
                    if self.actual_setting == 0 {
                        app_state.state = GameState::SelectingSong
                    } else if self.actual_setting == 1 {
                        app_state.state = GameState::Settings
                    } else if self.actual_setting == 2 {
                        app_state.state = GameState::Quitting
                    }
                },

                Event::Quit { .. } => {
                    app_state.is_running = false;
                },
                _ => {}
            }

           
        }
    }
}

