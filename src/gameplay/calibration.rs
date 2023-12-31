use std::time::{Duration, Instant};
use sdl2::{render::Canvas, video::Window, pixels::Color, ttf::Font, event::Event, keyboard::Keycode};
use crate::{key::GameKey, app::{App, AppState, GameState, CoordinationData}, game_object::GameObject, input::keybutton::KeyButton, input::button_module::Button};

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
        let calibration_note = GameKey::new(GameObject {active: true, x: ((app.width/2) - 75) as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, None);

        // UI ELEMENT
        let timer = Button::new(
            GameObject {
                active: true, x:(app.width - 40) as f32, y: 10.0, width: 0.0, height: 0.0},
            String::from("Timer"),
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
        );
        let enter_timer = Button::new(GameObject {active: true, x:(app.width - 40) as f32, y: 35.0, width: 0.0, height: 0.0},String::from("Timer"),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),);
        let out_timer = Button::new(GameObject { active: true, x:(app.width - 40) as f32, y: 60.0, width: 0.0, height: 0.0}, String::from("Timer"), Color::RGB(100, 100, 100),Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0));
        // controlers 
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 85) as f32, y: app.height as f32 - 160.0, width: 70.0, height: 70.0}, Color::RGB(200, 50, 100));

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

        app.canvas.set_draw_color(Color::BLACK);
        app.canvas.clear();
        
        let delta_time = self.delta_time(); // we use "delta time" on everything that moves on this update

        // timer
        let elapsed_time = self.start_time.elapsed();
        let milliseconds = elapsed_time.as_millis() / 10;
        self.timer.text = format!("{}", milliseconds);
        self.timer.render(&mut app.canvas, &app.texture_creator, &_font); // show timer
        self.enter_timer.render(&mut app.canvas, &app.texture_creator, &_font); // show timer
        self.out_timer.render(&mut app.canvas, &app.texture_creator, &_font); // show timer

        // buttons 
        let mut key_buttons = [&self.key_up];
        for button_key in key_buttons.iter_mut() {
            button_key.render(None, app);
        }

        Self::handle_notes(&mut self.started, &mut self.enter_timer, &mut self.out_timer, &mut self.calibration_note, milliseconds, delta_time, self.canvas_height, &mut app_state, app);
        Self::event_handler(&mut app_state,&mut event_pump);
        app.canvas.present();
    }

    fn event_handler(app_state: &mut AppState, event_pump: &mut sdl2::EventPump) {
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    app_state.state = GameState::MainMenu;
                }, 
                _ => {}
            }
        }
    }

    fn handle_notes(started: &mut bool, enter_timer: &mut Button, out_timer: &mut Button, note: &mut GameKey, milliseconds: u128, delta_time: Duration, height: u32, app_state: &mut AppState, app: &mut App) {
        let inside: bool;
        if note.mili < milliseconds {
            note.render(&mut app.canvas);
            note.update(delta_time, app.coordination_data.key_speed);         

            if (note.game_object.y > (height - 210) as f32) && (note.game_object.y < (height - 90) as f32) {
                inside = true;
            } else {
                inside = false;
            }

            if inside && *started {
                *started = false;
                enter_timer.text_color = Color::RED;
                app.coordination_data.base_time = milliseconds;
                enter_timer.text = format!("{}", app.coordination_data.base_time);
            } else if !inside && *started == false {
                *started = true;
                out_timer.text_color = Color::RED;
                app.coordination_data.end_time = milliseconds;
                out_timer.text = format!("{}", app.coordination_data.end_time);
            }

            if app.coordination_data.end_time > 0 && milliseconds >= app.coordination_data.end_time + 50 {
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