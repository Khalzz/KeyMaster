use std::{fs::File, io::{Error, Write}, time::Instant};
use sdl2::{pixels::Color, ttf::Font, event::Event, keyboard::Keycode};
use crate::{app::{App, AppState, GameController, GameState}, game_object::GameObject, input::button_module::{Button, TextAlign}};

pub struct GameLogic { // here we define the data we use on our script
    pub start_time: Instant,
    key_state: [bool;4],
    btn_list: Vec<Button>,
    back_button: Button
}

impl GameLogic {
    pub fn new(app: &mut App) -> Self {
        let key_left = Button::new(GameObject {active: true, x: ((app.width/2) - 185) as f32, y: app.height as f32 - 160.0, width: 70.0, height: 70.0},Some(String::from("Left")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);
        let key_right = Button::new(GameObject {active: true, x: ((app.width/2) + 115) as f32, y: app.height as f32 - 160.0, width: 70.0, height: 70.0},Some(String::from("Right")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);
        let key_up = Button::new(GameObject {active: true,x: ((app.width/2) - 85) as f32, y: app.height as f32 - 160.0, width: 70.0, height: 70.0},Some(String::from("Up")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);
        let key_bottom = Button::new(GameObject {active: true, x: ((app.width/2) + 15) as f32, y: app.height as f32 - 160.0, width: 70.0, height: 70.0},Some(String::from("Down")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);
        let back_button = Button::new(GameObject {active: true, x: 10.0 as f32, y: 10.0, width: 70.0, height: 30.0},Some(String::from("Back")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);

        let key_state = [false, false, false, false];
        let btn_list = vec![key_left, key_up, key_bottom, key_right];
        
        Self {
            start_time: Instant::now(),
            key_state,
            btn_list,
            back_button
        }
    }

    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App) {
        let mut texture_creator = app.canvas.texture_creator();
        for btn in 0..self.btn_list.len() {
            self.btn_list[btn].render(&mut app.canvas, &texture_creator, _font);
            self.back_button.render(&mut app.canvas, &mut texture_creator, _font);
            if self.key_state[btn] {
                self.btn_list[btn].text = Some(String::from("..."));
            } else {
                match Keycode::from_i32(app.play_keys[btn]) {
                    Some(keycode) => {
                        self.btn_list[btn].text = Some(keycode.to_string());
                    },
                    None => {},
                }
            }
        }
        Self::event_handler(&mut app_state,&mut event_pump, &mut self.key_state, &mut self.btn_list, &mut app.play_keys, &mut self.back_button);
    }

    fn event_handler(app_state: &mut AppState, event_pump: &mut sdl2::EventPump, key_state: &mut [bool;4], btn_list: &mut Vec<Button>, play_keys: &mut [i32;4], back_button: &mut Button) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    app_state.state = GameState::MainMenu;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    for btn in 0..btn_list.len() {
                        if key_state[btn] {
                            btn_list[btn].text = Some(String::from("..."));
                            play_keys[btn] = keycode as i32;

                            // save settings
                            Self::save_to_file(GameController{ controller_array: *play_keys }).expect("Something went wrong");
                        }
                    }
                    Self::reset(key_state);
                }
                _ => {}
            }

            for btn in 0..btn_list.len() {
                if btn_list[btn].on_click(&event) {
                    Self::reset(key_state);
                    key_state[btn] = true;
                }
            }
            if back_button.on_click(&event) {
                app_state.state = GameState::Settings
            }
            
        }
    }

    fn save_to_file(game_controller: GameController) -> Result<(), Error> {
        let mut file = File::create("settings.json")?;
        let serialized = serde_json::to_string(&game_controller)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }


    fn reset(key_state: &mut [bool;4]) {
        for key in 0..key_state.len() {
            key_state[key] = false;
        }
    }
}