use sdl2::{event::Event, keyboard::Keycode, mixer, pixels::Color, ttf::Font};
use crate::{ app::{self, App, AppState, GameState}, game_object::GameObject, input::{button_module::{Button, TextAlign}, slider_module::{self, Slider_input}}};

enum MenuSelector {
    Controller,
    Calibration,
    Exit
}

pub struct GameLogic<'a> { // here we define the data we use on our script
    opt_list: [&'a MenuSelector; 3],
    actual_opt: &'a MenuSelector,
    actual_setting: usize,
    btn_list: [Button;3],
    slider: Slider_input,
    pub started: bool
}

impl GameLogic<'_> {
    // here i define the buttons to use so in the update they are displayed
    pub fn new(app: &mut App) -> Self {
        let opt_list = [&MenuSelector::Controller, &MenuSelector::Calibration, &MenuSelector::Exit];

        // main menu
        let controller = Button::new(
            GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: 100.0, width: 100.0, height: 50.0},
            Some(String::from("Controller")),
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None, 
            TextAlign::Center
        );
        let calibration = Button::new(
            GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: 160.0, width: 100.0, height: 50.0},
            Some(String::from("Calibration")),
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None, 
            TextAlign::Center
        );
        let slider = Slider_input::new(
            app,
            GameObject {active: true, x:((app.width/2) - (200/2)) as f32, y: 300.0, width: 200.0, height: 10.0},
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            app.volume_percentage,
            false,
            Some(String::from("Audio")),
            true,
            100.0
        );

        let exit = Button::new(GameObject {active: true, x: 10.0 as f32, y: 10.0, width: 70.0, height: 30.0},Some(String::from("Back")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);

        let btn_list = [controller, calibration, exit];

        // at the end of our "new we need to return the data" since this is our constructor
        Self {
            opt_list,
            actual_opt: opt_list[0],
            actual_setting: 0,
            btn_list,
            slider,
            started: true
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        let mut texture_creator = app.canvas.texture_creator();

        if self.started == true {
            self.actual_setting = 0;
            self.started = false;
        }

        self.actual_opt = self.opt_list[self.actual_setting];
        for btn in 0..self.btn_list.len() {
            self.btn_list[btn].render(&mut app.canvas, &texture_creator, _font)
        }

        self.slider.render(app, _font);
        Self::event_handler(app_state, event_pump, &mut self.btn_list, &mut self.slider, app);
    }

    fn event_handler(app_state: &mut AppState, event_pump: &mut sdl2::EventPump,  btn_list: &mut [Button;3], slider: &mut Slider_input, app: &mut App) {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => {
                    app_state.is_running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    app_state.state = GameState::MainMenu;
                },
                _ => {}
            }

            mixer::Music::set_volume(((slider.is_hover(&event, app) as f32 / 100.0) * 128.0) as i32);   
            

            // change system of selecting options with arrows to on clicks
                if btn_list[0].on_click(&event) {
                    app_state.state = GameState::Controlers;
                }
                if btn_list[1].on_click(&event) {
                    app_state.state = GameState::Calibrating;
                    app.calibrate_on_start = false;
                }
                if btn_list[2].on_click(&event) {
                    app_state.state = GameState::MainMenu;
                }
        }
    }
}

