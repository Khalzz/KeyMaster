use std::{fs::{File, OpenOptions}, io::{Read, Write}, sync::MutexGuard, time::{Duration, Instant}};
use sdl2::{event::Event, keyboard::Keycode, mixer::{self, Music}, pixels::Color, rect::{Point, Rect}, render::Canvas, sys::KeyCode, ttf::Font, video::Window};
use serde_json::value;
use crate::{app::{App, AppState, GameState}, game_object::GameObject, input::{button_module::{Button, TextAlign}, keybutton::KeyButton}, key::{GameKey, KeyFlag}, load_song::Song};

const NUM_BARS: usize = 20;

#[derive(Clone,Debug,Copy)]
pub struct Note {
    pub state: bool,
    pub active: bool
}

pub struct KeyState {
    pub left: Note,
    pub top: Note,
    pub bottom: Note,
    pub right: Note
}

pub struct GameLogic<'a> { // here we define the data we use on our script
    last_frame: Instant,
    pub start_time: Instant,
    canvas_height: u32,
    key_left: KeyButton,
    key_up: KeyButton,
    key_bottom: KeyButton,
    key_right: KeyButton,
    key_state: KeyState,
    song_keys: Option<Vec<Vec<GameKey>>>,
    song_sync: i128,
    maked_song: Song,
    started_song: bool,
    started_level: bool,
    song: Option<Music<'a>>,
    points: u128,
    paused_time: Duration,
    error: bool,
    song_end: u128,
    end: bool,
    frame_count: u32,
    frame_timer: Duration,
    actual_button: usize,
    calibration: Button,
    controller: Button,
} 

impl GameLogic<'_> {
    // this is called once
    pub fn new(app: &mut App,  app_state: &mut AppState) -> Self {
        let mut song = None;
        let mut song_keys = None;
        let mut song_sync = 0;
        let mut song_end = 0;
        let mut error = false;
        app.alert_message = String::from("");
        app.paused = false;

        match &app_state.song_folder {
            Some(folder) => { 
                match mixer::Music::from_file("./songs/".to_owned() + folder + "/audio.mp3") {
                    Ok(song_ok) => song = Some(song_ok),
                    Err(_) => {
                        eprintln!("The song didn't loaded right for some reason: {}", folder);
                        app.alert_message = String::from("the song audio didn't loaded right");
                        app.paused = true;
                        error = true;
                    },
                }

                match &app.testing_song {
                    Some(testing) => {
                        song_keys = Some(testing.song.clone().get_keys(app, false));
                    },
                    None => {
                        let mut song_game: Song = Song {
                            name: String::from(""),
                            id: Some(0),
                            left_keys: vec![],
                            up_keys: vec![],
                            bottom_keys: vec![],
                            right_keys: vec![],
                            end: 0,
                            sync: Some(0)
                            
                        };
                        match Song::new(folder) {
                            Ok(song) => {
                                song_game = song
                            },
                            Err(_) => {
                                app.alert_message = String::from("the song didn't loaded right");
                                app.paused = true;
                                error = true;
                            },
                        }

                        match song_game.sync {
                            Some(value) => {
                                song_sync = value;
                            },
                            None => {},
                        }

                        song_end = song_game.end;
                        song_keys = Some(song_game.get_keys(app, false));
                    },
                }
            },
            None => {
                app.alert_message = String::from("the song didn't loaded right");
                app.paused = true;
                error = true;
            },
        }


        // controlers 
        let key_left = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 195) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0},Color::RGB(200, 50, 100));
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 95) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_bottom = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 5) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_right = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 105) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));

        // keys
        let mut ctrl_string = "".to_owned();

        for (i, key) in app.play_keys.iter().enumerate() {
            match Keycode::from_i32(*key) {
                Some(keycode) => {
                    let mut ctrl_opt = "";

                    if i == 1 {
                        ctrl_opt = "-[-], ";
                    } else if i == 2 {
                        ctrl_opt = "-[+] ";
                    }

                    if i == 1 || i == 2 {
                        ctrl_string += &(keycode.to_string() + ctrl_opt);
                    }
                },
                None => {},
            }
        }
        ctrl_string += &(", ESCAPE-[Back]");

        // buttons
        let key_state = KeyState { left: Note { state: false, active: true }, top: Note { state: false, active: true }, bottom: Note { state: false, active: true }, right: Note { state: false, active: true }};
        let calibration = Button::new(GameObject { active: true, x:0 as f32, y: 10.0, width: app.width as f32, height: 0.0}, Some(String::from("Calibration")), Color::RGB(100, 100, 100),Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0), None, TextAlign::Center);
        let controller = Button::new(GameObject { active: true, x:0 as f32, y: 40.0, width: app.width as f32, height: 0.0}, Some(String::from(ctrl_string)), Color::RGB(100, 100, 100),Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0), None, TextAlign::Center);

        Self {
            last_frame: Instant::now(),
            start_time: Instant::now(),
            key_left,
            key_up,
            key_bottom,
            key_right,
            key_state,
            song_keys,
            canvas_height: app.height,
            maked_song: Song { name: "Test".to_owned(), id: Some(0), left_keys: vec![], up_keys: vec!(), bottom_keys: vec![], right_keys: vec![], end: 0, sync: Some(0) },
            started_song: true,
            started_level: false,
            song,
            points: 0,
            paused_time: Duration::new(0, 0),
            error,
            song_end,
            end: false,
            frame_count: 0,
            frame_timer: Duration::new(0, 0),
            actual_button: 0,
            song_sync,
            calibration,
            controller
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App) {
        let delta_time = self.delta_time();
        let elapsed_time = self.start_time.elapsed() - self.paused_time;
        let mut milliseconds = 0;
                
        app.canvas.set_draw_color(Color::RGBA(40, 40, 40, 100));
        app.canvas.clear();

        match &app.testing_song {
            Some(_song) => {
                milliseconds = ((elapsed_time.as_millis() / 10) - (if app.paused_time > 0 { app.paused_time } else { 1 }) / 10) + ((_song.start_point) + 300.0) as u128
            },
            None => {
                milliseconds = (elapsed_time.as_millis() / 10) - (if app.paused_time > 0 { app.paused_time } else { 1 }) / 10
            },
        }

        let mut key_buttons = [&self.key_left, &self.key_up, &self.key_right, &self.key_bottom];
        for (i, button_key) in key_buttons.iter_mut().enumerate() {
            button_key.render(app, i);
        }

        match self.song_keys {
            Some(_) => Self::handle_notes(self, milliseconds, delta_time, app),
            None => {},
        }

        if !self.started_level {
            self.started_level = true;
            self.start_time = Instant::now();
        }
        
        self.calibration.text = Some(self.song_sync.to_string());
        self.calibration.render(&mut app.canvas, &app.texture_creator, &_font); 

        self.controller.render(&mut app.canvas, &app.texture_creator, &_font); 

        if milliseconds >= 300 && self.started_song == true {
            println!("{}", self.started_song);
            match app_state.state {
                GameState::SongCalibration => {
                    self.started_song = false;
                    match &self.song {
                        Some(song) => {
                            song.play(1).expect("The song didn't played");
                        },
                        None => {},
                    }
                },   
                _ => {}
            }
        }
        
        if milliseconds > self.song_end {
            Self::reset(app, app_state);
        }

        Self::event_handler(self, &mut app_state, &mut event_pump, app);
    }

    fn event_handler(&mut self, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[1]).unwrap() => {
                    self.song_sync -= 5;
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[2]).unwrap() => {
                    self.song_sync += 5;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.song_sync = 0;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    match &app_state.song_folder {
                        Some(folder) => {
                            let mut song_file = File::open("songs/".to_owned() + &folder + "/data.json").expect("The json didn't loaded correctly");
                            let mut json_string = String::new();
                            song_file.read_to_string(&mut json_string).expect("the json wasn't correctly read");
        
                            let mut song: Song = serde_json::from_str(&json_string).expect("The data wasnt read correctly");
        
                            song.sync = Some(self.song_sync);
        
                            let modified_json_string = serde_json::to_string_pretty(&song).expect("Something went wrong");
        
                            let mut file = OpenOptions::new().write(true).truncate(true).open("songs/".to_owned() + &folder + "/data.json").expect("Something went wrong");
                            file.write_all(modified_json_string.as_bytes()).expect("Something went wrong");
        
                            Self::reset(app, app_state);
                        },
                        None => {},
                    }
                    

                }, Event::Quit { .. } => {
                    app_state.is_running = false;
                } 
                _ => {}
            }
        }
    }

    fn pause(app: &mut App) {
        app.paused = true;
        mixer::Music::pause();
        app.start_pause = Instant::now();
        app.coordination_data.key_speed = 0.0;
    }

    fn unpause(app: &mut App) {
        app.paused = false;
        mixer::Music::resume();
        app.coordination_data.key_speed = app.coordination_data.saved_key_speed;
        app.paused_time += app.start_pause.elapsed().as_millis();
    }

    fn reset(app: &mut App, app_state: &mut AppState) {
        app.reseted = false;
        Self::unpause(app);
        mixer::Music::halt();
        app_state.state = GameState::SelectingSong;
    }

    fn handle_notes(&mut self, milliseconds: u128, delta_time: Duration, app: &mut App) {
        

        if let Some(song_keys) = &mut self.song_keys {
            for key_index in 0..4 {
                let mut inside = false;
                let actual_key = match key_index {
                    0 => &mut self.key_left,
                    1 => &mut self.key_up,
                    2 => &mut self.key_bottom,
                    _ => &mut self.key_right,
                };

                let mut remove: Vec<usize> = Vec::new(); // Collect indices of notes to remove

                for (i, note) in song_keys[key_index].iter_mut().enumerate() {
                    if note.game_object.y > app.width as f32 {
                        remove.push(i);
                    }

                    if note.mili - (app.coordination_data.base_time - self.song_sync) < milliseconds.try_into().unwrap() {
                        note.render(app);
                        note.update(delta_time, app.coordination_data.key_speed);

                        if (note.game_object.y > (self.canvas_height - 150) as f32) && (note.game_object.y < (self.canvas_height - 80) as f32) {
                            inside = true;
                        }
                    }
                }

                if inside {
                    actual_key.state = 2;
                } else {
                    actual_key.state = 0;
                }
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