use std::time::{Duration, Instant};
use sdl2::{pixels::Color, ttf::Font, event::Event, keyboard::Keycode, mixer::{self, Music}};
use crate::{app::{App, AppState, GameState}, game_object::GameObject, input::{button_module::Button, keybutton::KeyButton}, key::GameKey, load_song::Song};

pub struct KeyState {
    pub left: bool,
    pub top: bool,
    pub bottom: bool,
    pub right: bool
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
    maked_song: Song,
    started_song: bool,
    song: Option<Music<'a>>,
    points: u128,
    pause_elements: Vec<Button>,
    ui_elements: Vec<Button>,
    end_elements: Vec<Button>,
    paused_time: Duration,
    error: bool,
    error_elements: Vec<Button>,
    song_end: u128,
    end: bool
} 

impl GameLogic<'_> {
    // this is called once
    pub fn new(app: &mut App,  app_state: &mut AppState) -> Self {
        let mut song = None;
        let mut song_keys = None;
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
                            left_keys: vec![],
                            up_keys: vec![],
                            bottom_keys: vec![],
                            right_keys: vec![],
                            end: 0,
                            
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

        // UI ELEMENT
        let ui_points = Button::new(GameObject { active: true, x:((app.width/2) - 70 ) as f32, y: 10.0, width: 140.0, height: 30.0}, Some(String::from("Points")),Color::RGB(200, 100, 100), Color::WHITE, Color::RGB(200, 10, 0), Color::RGB(200, 0, 0),None);

        let timer = Button::new(GameObject {active: true, x:(app.width - 40) as f32, y: 30.0, width: 0.0, height: 0.0},Some(String::from("Timer")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None);

        // PAUSE UI
        let pause_text = Button::new(GameObject {active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32},Some(String::from("Pause")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None);
        let resume = Button::new(GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height - (app.height / 2) + 50) as f32, width: 100.0, height: 50.0},Some(String::from("resume")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None);
        let exit = Button::new(GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height - (app.height / 2) + 110) as f32, width: 100.0, height: 50.0},Some(String::from("exit")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None);

        // Error UI
        let error_text = Button::new(GameObject {active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32},Some(app.alert_message.clone()),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None);
        let ok_button = Button::new(GameObject { active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height as f32/2.0) + 200.0 as f32, width: 100.0, height: 50.0},Some(String::from("OK")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None);

        // End UI
        let end_text = Button::new(GameObject {active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32},Some(String::from("Congrats!!!")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None);
        let back_to_menu = Button::new(GameObject {active: true, x:((app.width/2) - (160/2)) as f32, y: (app.height - (app.height / 2) + 50) as f32, width: 160.0, height: 50.0},Some(String::from("Back to menu")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None);

        // UI LISTS
        let ui_elements = vec![ui_points, timer];
        let pause_elements = vec![pause_text, resume, exit];
        let end_elements = vec![end_text, back_to_menu];
        let error_elements = vec![error_text, ok_button];

        // controlers 
        let key_left = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 195) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0},Color::RGB(200, 50, 100));
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 95) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_bottom = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 5) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_right = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 105) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));

        // buttons
        let key_state = KeyState { left: false, top: false, bottom: false, right: false};

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
            maked_song: Song { name: "Test".to_owned(), left_keys: vec![], up_keys: vec!(), bottom_keys: vec![], right_keys: vec![], end: 0 },
            started_song: true,
            song,
            points: 0,
            paused_time: Duration::new(0, 0),
            pause_elements,
            ui_elements,
            end_elements,
            error,
            error_elements,
            song_end,
            end: false
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App) {
        match app_state.song_folder {
            Some(_) => {
                let delta_time = self.delta_time(); // we use "delta time" on everything that moves on this update

                // timer
                let elapsed_time = self.start_time.elapsed() - self.paused_time;
                let mut milliseconds = 0;
                
                if app.paused && !self.end{
                    milliseconds = 0;
                    if self.error == true{
                        for button in &self.error_elements {
                            button.render(&mut app.canvas, &app.texture_creator, &_font);
                        }
                    } else {
                        app.canvas.set_draw_color(Color::RGBA(29, 91, 88, 100));
                        app.canvas.clear();
                        for button in &self.pause_elements {
                            button.render(&mut app.canvas, &app.texture_creator, &_font);
                        }
                    }
                } else {
                    if self.end == true {
                        app.canvas.set_draw_color(Color::RGBA(29, 91, 88, 100));
                        app.canvas.clear();
                        self.end_elements[0].text = Some("Congrats, you got ".to_owned() + &self.points.to_string().to_owned() + " points");
                        for button in &self.end_elements {
                            button.render(&mut app.canvas, &app.texture_creator, &_font);
                        }
                    } else {
                        // clearing
                        app.canvas.set_draw_color(Color::RGBA(29, 91, 88, 100));
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
                        for button_key in key_buttons.iter_mut() {
                            button_key.render(Some("assets/sprites/WhiteKey-Sheet.png"), app);
                        }

                        match self.song_keys {
                            Some(_) => Self::handle_notes(self, milliseconds, delta_time, app),
                            None => {},
                        } 

                        self.ui_elements[0].text = Some(self.points.to_string()); // point text
                        self.ui_elements[1].text = Some(format!("{}", milliseconds)); // timer

                            for button in &self.ui_elements {
                                button.render(&mut app.canvas, &app.texture_creator, &_font);
                            }
                        

                        // audio loading and playing
                        if milliseconds >= 300 && self.started_song == true {
                            match app_state.state {
                                GameState::Playing => {
                                    self.started_song = false;
                                    match &self.song {
                                        Some(song) => {
                                            song.play(1);
                                            match &app.testing_song {
                                                Some(testing) => {
                                                    match mixer::Music::set_pos(((elapsed_time.as_millis() / 10) - app.paused_time / 10) as f64 + (testing.start_point) as f64 / 100.0) {
                                                        Ok(_) => {},
                                                        Err(_) => {
                                                            app.alert_message = String::from("the song position wasn't loaded correctly");
                                                            app.paused = true;
                                                            self.error = true;
                                                        },
                                                    };
                                                },
                                                None => {},
                                            }
                                        },
                                        None => {},
                                    }
                                },   
                                _ => {}
                            }
                        }
                    }

                } 

                Self::event_handler(self, milliseconds, &mut app_state, &mut event_pump, app);
            },
            None => {},
        }
        
    }

    fn event_handler(&mut self, milliseconds: u128,app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        for event in event_pump.poll_iter() {
            match event { 
                Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    match app.testing_song {
                        Some(_) => {
                            Self::reset(app, app_state);
                            app_state.state = GameState::Editing
                        },
                        None => {    
                            if !app.paused {
                                Self::pause(app);
                            } else {
                                Self::unpause(app);
                            }
                        },
                    }
                },Event::KeyDown { keycode: Some(Keycode::R), .. }  => {
                    // mixer::Music::set_pos(10.0);
                }, Event::Quit { .. } => {
                    app_state.is_running = false;
                } 
                _ => {}
            }
            
            if app.paused && !self.error {
                if self.pause_elements[1].on_click(&event) {
                    Self::unpause(app);
                } 
                if self.pause_elements[2].on_click(&event) {
                    Self::reset(app, app_state);
                }
            } else if app.paused && self.error {
                if self.error_elements[1].on_click(&event) {
                    Self::reset(app, app_state)
                }
            } else if self.end{
                if self.end_elements[1].on_click(&event) {
                    Self::reset(app, app_state)
                }
            }

            self.key_state.left = self.key_left.update(&mut self.maked_song, milliseconds ,&event, app.play_keys[0],&mut app.play_keys);
            self.key_state.top = self.key_up.update(&mut self.maked_song, milliseconds,&event, app.play_keys[1],&mut app.play_keys);
            self.key_state.bottom = self.key_bottom.update(&mut self.maked_song,milliseconds,&event, app.play_keys[2],&mut app.play_keys);
            self.key_state.right = self.key_right.update(&mut self.maked_song,milliseconds,&event, app.play_keys[3],&mut app.play_keys);
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
            for keys in 0..4 {
                let actual_key = match keys {
                    0 => Some(self.key_state.left),
                    1 => Some(self.key_state.top),
                    2 => Some(self.key_state.bottom),
                    3 => Some(self.key_state.right),
                    _ => None
                };
    
                for note in song_keys[keys].iter_mut() {
                    if milliseconds > self.song_end {
                        self.end = true;
                        mixer::Music::pause();
                    }

                    if note.mili < milliseconds {
                        note.render(&mut app.canvas);
                        note.update(delta_time, app.coordination_data.key_speed);
    
                        if actual_key.is_some() {
                            let actual_key = actual_key.unwrap();
    
                            if (note.game_object.y > (self.canvas_height - 210) as f32) && (note.game_object.y < (self.canvas_height - 90) as f32) && actual_key {
                                if note.game_object.active {
                                    if note.holding == true {
                                        self.points += 1; // its not calling aparently
                                    } else {
                                        self.points += 100; 
                                    }
                                }
                                note.game_object.active = false;
                                match keys {
                                    0 => self.key_left.state = 2,
                                    1 => self.key_up.state = 2,
                                    2 => self.key_bottom.state = 2,
                                    3 => self.key_right.state = 2,
                                    _ => {}
                                }
                            }
                        }
                    }
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