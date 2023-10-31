use std::time::{Duration, Instant};
use sdl2::{render::{Canvas, TextureCreator}, video::{Window, WindowContext}, pixels::Color, ttf::Font, event::Event, keyboard::Keycode, image::LoadTexture, mixer::{self, Music}};
use crate::{key::GameKey, app::{App, AppState, GameState, self}, game_object::GameObject, input::keybutton::{KeyButton}, input::button_module::Button, load_song::Song};

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
    paused: bool,
    paused_time: Duration,
    total_time: Duration
} 

impl GameLogic<'_> {
    // this is called once
    pub fn new(app: &mut App,  app_state: &mut AppState) -> Self {
        let mut song = None;

        let mut song_keys = None;
        match &app_state.song_folder {
            Some(folder) => {
                song = Some(mixer::Music::from_file("./songs/".to_owned() + folder + "/audio.mp3").expect("Not found"));
                println!("{}", folder);

                match &app.testing_song {
                    Some(testing) => {
                        let song_game = Song::new(folder);
                        song_keys = Some(testing.clone().get_keys(&mut app.width, &app.coordination_data.base_time, app.coordination_data.key_speed));
                    },
                    None => {
                        let song_game = Song::new(folder);
                        song_keys = Some(song_game.get_keys(&mut app.width, &app.coordination_data.base_time, app.coordination_data.key_speed));
                    },
                }
                
            },
            None => {},
        }

        // UI ELEMENT
        let ui_points = Button::new(GameObject { active: true, x:(app.width - 40) as f32, y: 10.0, width: 0.0, height: 0.0}, String::from("Points"), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0),);
        let timer = Button::new(GameObject {active: true, x:(app.width - 40) as f32, y: 30.0, width: 0.0, height: 0.0},String::from("Timer"),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),);

        // PAUSE UI
        let pause_text = Button::new(GameObject {active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32},String::from("Pause"),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),);
        let resume = Button::new(GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height - (app.height / 2) + 50) as f32, width: 100.0, height: 50.0},String::from("resume"),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),);
        let exit = Button::new(GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height - (app.height / 2) + 110) as f32, width: 100.0, height: 50.0},String::from("exit"),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),);

        // UI LISTS
        let ui_elements = vec![ui_points, timer];
        let pause_elements = vec![pause_text, resume, exit];

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
            paused: false,
            paused_time: Duration::new(0, 0),
            total_time: Duration::new(0, 0),
            pause_elements,
            ui_elements
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App) {
        match app_state.song_folder {
            Some(_) => {
                app.canvas.set_draw_color(Color::RGBA(29, 91, 88, 100));
                app.canvas.clear();
                
                let delta_time = self.delta_time(); // we use "delta time" on everything that moves on this update

                // timer
                let elapsed_time = self.start_time.elapsed() - self.paused_time;
                let mut milliseconds = (elapsed_time.as_millis() / 10) - app.paused_time / 10;

                // buttons 
                let mut key_buttons = [&self.key_left, &self.key_up, &self.key_right, &self.key_bottom];
                for button_key in key_buttons.iter_mut() {
                    button_key.render(Some("assets/sprites/WhiteKey-Sheet.png"), app);
                }

                match self.song_keys {
                    Some(_) => Self::handle_notes(self, milliseconds, delta_time, app),
                    None => {},
                } 

                self.ui_elements[0].text = self.points.to_string(); // point text
                self.ui_elements[1].text = format!("{}", milliseconds); // timer

                if app.paused {
                    milliseconds = 0;
                    for button in &self.pause_elements {
                        button.render(&mut app.canvas, &app.texture_creator, &_font);
                    }
                } else {
                    for button in &self.ui_elements {
                        button.render(&mut app.canvas, &app.texture_creator, &_font);
                    }
                }

                // audio loading and playing
                if milliseconds >= 300 && self.started_song == true {
                    match app_state.state {
                        GameState::Playing => {
                            self.started_song = false;
                            match &self.song {
                                Some(song) => {
                                    song.play(1);
                                },
                                None => {},
                            }
                        },   
                        _ => {}
                    }
                }

                Self::event_handler(self, milliseconds, &mut app_state, &mut event_pump, app);
                app.canvas.present();
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
                            // reset
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
                }, Event::Quit { .. } => {
                    app_state.is_running = false;
                } 
                _ => {}
            }
            
            if app.paused {
                if self.pause_elements[1].on_click(&event) {
                    Self::unpause(app);
                } 
                if self.pause_elements[2].on_click(&event) {
                    Self::reset(self, app, app_state);
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

    fn reset(&mut self, app: &mut App, app_state: &mut AppState) {
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
                    if note.mili < milliseconds {
                        note.render(&mut app.canvas);
                        note.update(delta_time, app.coordination_data.key_speed);
                        
                        if actual_key.is_some() {
                            let actual_key = actual_key.unwrap();
                            
                            if (note.game_object.y > (self.canvas_height - 210) as f32) && (note.game_object.y < (self.canvas_height - 90) as f32) && actual_key == true {
                                self.points += 100; // its getting called a lot of times
                                note.game_object.active = false;
                                if keys == 0 {
                                    self.key_left.state = 2
                                } else if keys == 1 {
                                    self.key_up.state = 2
                                } else if keys == 2 {
                                    self.key_bottom.state = 2
                                } else if keys == 3 {
                                    self.key_right.state = 2
                                }
                            }
                        }
                    }
                }
            }
        } else {
        }
    }

    fn delta_time(&mut self) -> Duration {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_frame); // this is our Time.deltatime
        self.last_frame = current_time;
        return delta_time
    }
}