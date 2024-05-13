use std::time::{Duration, Instant};
use sdl2::{pixels::Color, ttf::Font, event::Event, keyboard::Keycode};
use crate::{app::{App, AppState, GameState}, game_object::GameObject, input::{button_module::{Button, TextAlign}, keybutton::KeyButton}, key::GameKey};

 pub struct GameLogic { // here we define the data we use on our script
    last_frame: Instant,
    pub start_time: Instant,
    timer: Button,
    enter_timer: Button,
    out_timer: Button,
    canvas_height: u32,
    key_up: KeyButton,
    calibration_note: GameKey,
    started: bool
}

impl GameLogic {
    // this is called once
    pub fn new(app: &mut App) -> Self {
        let calibration_note = GameKey::new(GameObject {active: true, x: ((app.width/2) - 75) as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as i128, None, None, false);

        // UI ELEMENT
        let timer = Button::new(GameObject {active: true, x:(app.width - 40) as f32, y: 10.0, width: 0.0, height: 0.0},Some(String::from("Timer")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None,TextAlign::Center);
        let enter_timer = Button::new(GameObject {active: true, x:0 as f32, y: app.height as f32 - 160.0, width: app.width as f32, height: 10.0},Some(String::from("Timer")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);
        let out_timer = Button::new(GameObject { active: true, x:0 as f32, y: app.height as f32 - 90.0, width: app.width as f32, height: 0.0}, Some(String::from("Timer")), Color::RGB(100, 100, 100),Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0), None, TextAlign::Center);
        // controlers 
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 95) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));

        Self {
            last_frame: Instant::now(),
            start_time: Instant::now(),
            timer,
            enter_timer,
            out_timer,
            key_up,
            calibration_note,
            canvas_height: app.height,
            started: true
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App) {
        let texture_creator = app.canvas.texture_creator();
        let delta_time = self.delta_time();

        // timer
        let elapsed_time = self.start_time.elapsed();
        let milliseconds = elapsed_time.as_millis() / 10;
        self.timer.text = Some(format!("{}", milliseconds));
        self.timer.render(&mut app.canvas, &texture_creator, &_font); 
        self.enter_timer.render(&mut app.canvas, &texture_creator, &_font); 
        self.out_timer.render(&mut app.canvas, &texture_creator, &_font); 

        // buttons 
        let mut key_buttons = [&self.key_up];
        for button_key in key_buttons.iter_mut() {
            button_key.render(app, 0);
        }

        Self::handle_notes(&mut self.started, &mut self.enter_timer, &mut self.out_timer, &mut self.calibration_note, milliseconds, delta_time, self.canvas_height, &mut app_state, app);
        Self::event_handler(&mut app_state,&mut event_pump, app);
    }

    fn event_handler(app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    if !app.calibrate_on_start {
                        app_state.state = GameState::MainMenu;
                    }
                }, 
                _ => {}
            }
        }
    }

    fn handle_notes(started: &mut bool, enter_timer: &mut Button, out_timer: &mut Button, note: &mut GameKey, milliseconds: u128, delta_time: Duration, height: u32, app_state: &mut AppState, app: &mut App) {
        let inside: bool;
        if note.mili < milliseconds.try_into().unwrap() {
            note.render(app);
            note.update(delta_time, app.coordination_data.key_speed);         

            if (note.game_object.y > (height - 150) as f32) && (note.game_object.y < (height - 80) as f32) {
                inside = true;
            } else {
                inside = false;
            }

            if inside && *started {
                *started = false;
                enter_timer.text_color = Color::RED;
                app.coordination_data.base_time = milliseconds as i128;
                enter_timer.text = Some(format!("{}", app.coordination_data.base_time));
            } else if !inside && *started == false {
                *started = true;
                out_timer.text_color = Color::RED;
                app.coordination_data.end_time = milliseconds as i128;
                out_timer.text = Some(format!("{}", app.coordination_data.end_time));
            }

            if app.coordination_data.end_time > 0 && milliseconds >= (app.coordination_data.end_time + 50).try_into().unwrap() {
                app_state.state = GameState::MainMenu;
            }
        }
    }
    
    fn delta_time(&mut self) -> Duration {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_frame); // this is our Time.deltatime
        self.last_frame = current_time;
        return delta_time
    }
}