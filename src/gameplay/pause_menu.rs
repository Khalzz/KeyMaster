use std::{sync::MutexGuard, time::{Duration, Instant}};
use sdl2::{event::Event, keyboard::Keycode, mixer::{self, Music}, pixels::Color, rect::{Point, Rect}, render::Canvas, ttf::Font, video::Window};
use crate::{app::{App, AppState, GameState}, game_object::GameObject, input::{button_module::{Button, TextAlign}, keybutton::KeyButton}, key::GameKey, load_song::Song};

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
    end: bool,
    frame_count: u32,
    frame_timer: Duration,
    fps: u32,
    combo: Button,
    combo_val: u32,
    max_combo: u32,
    actual_button: usize
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
                        /*
                        thread::spawn(move || {
                            let _ = sdl2::messagebox::show_simple_message_box(
                                MessageBoxFlag::INFORMATION,
                                "Ups",
                                "the song audio didn't loaded right",
                                None,
                            );
                        });
                        app.reseted = false;
                        Self::unpause(app);
                        mixer::Music::halt();
                        app_state.state = GameState::SelectingSong;
                        */
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
        let ui_points = Button::new(GameObject { active: true, x:((app.width/2) - 70 ) as f32, y: 10.0, width: 140.0, height: 30.0}, Some(String::from("Points")),Color::RGB(200, 100, 100), Color::WHITE, Color::RGB(200, 10, 0), Color::RGB(200, 0, 0),None, TextAlign::Center);
        let timer = Button::new(GameObject {active: true, x:10 as f32, y: 30.0, width: 0.0, height: 0.0},Some(String::from("Timer")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Left);
        let framerate = Button::new(GameObject {active: true, x:10 as f32, y: 10.0, width: 0.0, height: 0.0},Some(String::from("Framerate")),Color::RGBA(100, 100, 100, 0),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Left);
        let combo = Button::new(GameObject {active: true, x:(app.width/2) as f32, y: (app.height/2) as f32, width: 0.0, height: 0.0},Some(String::from("10 Combo")),Color::RGBA(100, 100, 100, 0),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);

        // PAUSE UI
        let pause_text = Button::new(GameObject {active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32},Some(String::from("Pause")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None, TextAlign::Center);
        let resume = Button::new(GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height - (app.height / 2) + 50) as f32, width: 100.0, height: 50.0},Some(String::from("resume")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None, TextAlign::Center);
        let exit = Button::new(GameObject {active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height - (app.height / 2) + 110) as f32, width: 100.0, height: 50.0},Some(String::from("exit")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None, TextAlign::Center);

        // Error UI
        let error_text = Button::new(GameObject {active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32},Some(app.alert_message.clone()),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None, TextAlign::Center);
        let ok_button = Button::new(GameObject { active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height as f32/2.0) + 200.0 as f32, width: 100.0, height: 50.0},Some(String::from("OK")),Color::RGB(100, 100, 100),Color::WHITE,Color::RGB(0, 200, 0),Color::RGB(0, 0, 0),None, TextAlign::Center);

        // End UI
        let end_text = Button::new(GameObject {active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32},Some(String::from("Congrats!!!")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None, TextAlign::Center);
        let combo_text = Button::new(GameObject {active: true, x: 0.0, y: 50.0, width: app.width as f32, height: app.height as f32},Some(String::from("Combo")),Color::RGBA(0, 0, 0, 0),Color::WHITE,Color::RGBA(0, 0, 0,0),Color::RGBA(0, 0, 0,0),None, TextAlign::Center);
        let back_to_menu = Button::new(GameObject {active: true, x:((app.width/2) - (160/2)) as f32, y: (app.height - (app.height / 2) + 100) as f32, width: 160.0, height: 50.0},Some(String::from("Back to menu")),Color::RGBA(0, 0, 0, 200),Color::WHITE,Color::RGBA(0, 200, 0,0),Color::RGBA(0, 0, 0,0),None, TextAlign::Center);

        // UI LISTS
        let ui_elements = vec![ui_points, timer, framerate];
        let pause_elements = vec![pause_text, resume, exit];
        let end_elements = vec![end_text, combo_text, back_to_menu];
        let error_elements = vec![error_text, ok_button];

        // controlers 
        let key_left = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 195) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0},Color::RGB(200, 50, 100));
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 95) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_bottom = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 5) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_right = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 105) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));

        // buttons
        let key_state = KeyState { left: Note { state: false, active: true }, top: Note { state: false, active: true }, bottom: Note { state: false, active: true }, right: Note { state: false, active: true }};

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
            maked_song: Song { name: "Test".to_owned(), id: Some(0), left_keys: vec![], up_keys: vec!(), bottom_keys: vec![], right_keys: vec![], end: 0 },
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
            end: false,
            frame_count: 0,
            frame_timer: Duration::new(0, 0),
            fps: 0,
            combo,
            combo_val: 0,
            max_combo: 0,
            actual_button: 0
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App, fft_data: MutexGuard<'_, Vec<f32>>) {
        let texture_creator = app.canvas.texture_creator();
        match app_state.song_folder {
            Some(_) => {
                if app.paused && !self.end{ // pause state
                    milliseconds = 0;
                    if self.error == true{
                        for (i, button) in self.error_elements.iter_mut().enumerate() {
                            if i == self.actual_button {
                                button.color = Color::RGB(0, 200, 0);
                            } else {
                                button.color = Color::RGB(100, 100, 100);
                            }
                            button.render(&mut app.canvas, &texture_creator, &_font);
                        }
                    } else {
                        for (i, button) in self.pause_elements.iter_mut().enumerate() {
                            if i == self.actual_button && i != 0 {
                                button.color = Color::RGB(0, 200, 0);
                            } else {
                                button.color = Color::RGB(100, 100, 100);
                            }
                            button.render(&mut app.canvas, &texture_creator, &_font);
                        }
                    }
                }
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
                                self.actual_button = 0;
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
                if self.end_elements[2].on_click(&event) {
                    Self::reset(app, app_state)
                }
            }  
            
            self.key_state.left.state = self.key_left.update(&mut self.maked_song, milliseconds ,&event, app.play_keys[0], &mut app.play_keys);
            self.key_state.top.state = self.key_up.update(&mut self.maked_song, milliseconds,&event, app.play_keys[1], &mut app.play_keys);
            self.key_state.bottom.state = self.key_bottom.update(&mut self.maked_song,milliseconds,&event, app.play_keys[2], &mut app.play_keys);
            self.key_state.right.state = self.key_right.update(&mut self.maked_song,milliseconds,&event, app.play_keys[3], &mut app.play_keys);
        }
    }

    fn display_framerate(&mut self, delta_time: Duration) {
        self.frame_count += 1;
        self.frame_timer += delta_time;

        // Calculate FPS every second
        if self.frame_timer >= Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.frame_timer -= Duration::from_secs(1); // Remove one second from the timer
        }

        // Render FPS text
        let fps_text = format!("FPS: {}", self.fps);
        self.ui_elements[2].text = Some(fps_text);
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

                    if note.mili < milliseconds {
                        note.render(app);
                        note.update(delta_time, app.coordination_data.key_speed);

                        if (note.game_object.y + 50.0 > (self.canvas_height - 150) as f32) && (note.game_object.y < (self.canvas_height - 80) as f32) && actual_key.pressed && actual_key.timer_hold.elapsed().as_millis() / 10 < 10 {
                            if note.game_object.active {
                                if note.holding {
                                    actual_key.timer_hold = Instant::now();
                                    self.points += 1;
                                } else {
                                    self.combo_val += 1;
                                    if self.combo_val > self.max_combo {
                                        self.max_combo = self.combo_val;
                                    }
                                    self.points += 100;
                                }
                            }
                            note.game_object.active = false;
                            actual_key.state = 2;
                        } else if (note.game_object.y > (self.canvas_height - 80) as f32) && !note.muted && note.game_object.active {
                            if !note.muted {
                                self.combo_val = 0;
                            }
                            note.muted = true;                          
                        }
                    }
                }
                for value in remove.iter() {
                    song_keys[key_index].remove(*value);
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