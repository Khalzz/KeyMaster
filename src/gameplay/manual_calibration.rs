use std::time::{Duration, Instant};
use sdl2::{event::Event, keyboard::Keycode, mixer::{self, Music}, pixels::Color, render::Canvas, ttf::Font, video::Window};
use crate::{app::{self, App, AppState, CoordinationData, GameState}, game_object::GameObject, input::{button_module::{Button, TextAlign}, keybutton::KeyButton}, key::GameKey};

 pub struct GameLogic<'a> { // here we define the data we use on our script
    last_frame: Instant,
    pub start_time: Instant,
    timer: Button,
    enter_timer: Button,
    out_timer: Button,
    calibration: Button,
    canvas_height: u32,
    key_up: KeyButton,
    calibration_notes: Vec<GameKey>,
    started: bool,
    song: Option<Music<'a>>,
}

impl GameLogic<'_> {
    // this is called once
    pub fn new(app: &mut App) -> Self {
        let mut song = None;
    
        match mixer::Music::from_file("./assets/audio/calibration.mp3") {
            Ok(song_ok) => song = Some(song_ok),
            Err(_) => {},
        }

        let mut calibration_notes: Vec<GameKey> = vec![];
        let mut milis = 300;

        for note in 0..100 {
            let calibration_note = GameKey::new(GameObject {active: true, x: ((app.width/2) - 75) as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, milis as i128, None, None, false);
            calibration_notes.push(calibration_note);
            milis += 400
        }

        // UI ELEMENT
        let timer = Button::new(GameObject {active: true, x:(app.width - 40) as f32, y: 10.0, width: 0.0, height: 0.0},Some(String::from("Timer")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None,TextAlign::Center);
        let enter_timer = Button::new(GameObject {active: true, x:0 as f32, y: app.height as f32 - 150.0, width: app.width as f32, height: 10.0},Some(String::from("Timer")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);
        let out_timer = Button::new(GameObject { active: true, x:0 as f32, y: app.height as f32 - 80.0, width: app.width as f32, height: 0.0}, Some(String::from("Timer")), Color::RGB(100, 100, 100),Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0), None, TextAlign::Center);
        let calibration = Button::new(GameObject { active: true, x:0 as f32, y: 10.0, width: app.width as f32, height: 0.0}, Some(String::from("Calibration")), Color::RGB(100, 100, 100),Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0), None, TextAlign::Center);
        
        // controlers 
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 95) as f32, y: app.height as f32 - 160.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));

        Self {
            last_frame: Instant::now(),
            start_time: Instant::now(),
            timer,
            enter_timer,
            out_timer,
            calibration,
            key_up,
            calibration_notes,
            canvas_height: app.height,
            started: false,
            song,
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App) {
        let mut texture_creator = app.canvas.texture_creator();
        let delta_time = self.delta_time();

        /*         
        match &self.song {
            Some(song) => {
                if self.started == false {
                    song.play(-1).expect("The song didn't played");
                    self.started = true;
                }
            },
            None => todo!(),
        }
        */

        


        // timer
        let elapsed_time = self.start_time.elapsed();
        let milliseconds = elapsed_time.as_millis() / 10;
        self.timer.text = Some(format!("{}", milliseconds));
        self.timer.render(&mut app.canvas, &texture_creator, &_font); 
        self.enter_timer.render(&mut app.canvas, &texture_creator, &_font); 
        self.out_timer.render(&mut app.canvas, &texture_creator, &_font); 

        self.calibration.text = Some(app.coordination_data.base_time.to_string());
        self.calibration.render(&mut app.canvas, &texture_creator, &_font); 

        // buttons 
        let mut key_buttons = [&self.key_up];
        for button_key in key_buttons.iter_mut() {
            button_key.render(app, 0);
        }

        Self::handle_notes(self, delta_time, milliseconds, app);
        Self::event_handler(&mut app_state,&mut event_pump, app);
    }

    fn event_handler(app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        for event in event_pump.poll_iter() {
            match event { 
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[1]).unwrap() => {
                    if app.coordination_data.base_time > 10 {
                        app.coordination_data.base_time -= 10;
                    } else {
                        app.coordination_data.base_time = 0;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[2]).unwrap() => {
                    app.coordination_data.base_time += 10;
                },
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    mixer::Music::pause();
                    app_state.state = GameState::MainMenu;
                }, 
                _ => {}
            }
        }
    }

    pub fn handle_notes(&mut self, delta_time: Duration, milliseconds: u128,  app: &mut App) {
        let inside: bool;
        
        if milliseconds % 300 == 0 {
            match &self.song {
                Some(song) => {
                    if self.started == false {
                        song.play(1).expect("The song didn't played");
                    }
                },
                None => todo!(),
            }
        }

        for key in self.calibration_notes.iter_mut() {
            if key.mili - app.coordination_data.base_time < milliseconds.try_into().unwrap() {
                key.render(app);
                key.update(delta_time, app.coordination_data.key_speed);         

                if (key.game_object.y > (self.canvas_height - 150) as f32) && (key.game_object.y < (self.canvas_height - 80) as f32) {
                    self.key_up.state = 2;
                } else {
                    self.key_up.state = 0;
                }

                /* 
                if inside && *started {
                    *started = false;
                    enter_timer.text_color = Color::RED;
                    app.coordination_data.base_time = milliseconds;
                    enter_timer.text = Some(format!("{}", app.coordination_data.base_time));
                } else if !inside && *started == false {
                    *started = true;
                    out_timer.text_color = Color::RED;
                    app.coordination_data.end_time = milliseconds;
                    out_timer.text = Some(format!("{}", app.coordination_data.end_time));
                }

                if app.coordination_data.end_time > 0 && milliseconds >= app.coordination_data.end_time + 50 {
                    app_state.state = GameState::MainMenu;
                }
                */
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