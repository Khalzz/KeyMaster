use std::{sync::MutexGuard, time::{Duration, Instant}};
use sdl2::{event::Event, keyboard::Keycode, mixer::{self, Music}, pixels::Color, rect::{Point, Rect}, render::Canvas, ttf::Font, video::Window};
use crate::{app::{App, AppState, GameState}, game_object::{self, GameObject}, input::{button_module::{Button, TextAlign}, keybutton::KeyButton}, key::GameKey, load_song::Song};

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
    actual_button: usize,
    ui_texts: Vec<Button>,
    bpm_timer: Instant,
    bpm_bars: Vec<BeatLine>
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
                            sync: Some(0),
                            bpm: Some(0)
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
        let pause_elements = vec![resume, exit];
        let end_elements = vec![back_to_menu];
        let error_elements = vec![ok_button];
        let ui_texts = vec![pause_text, end_text, combo_text, error_text];

        // controlers 
        let key_left = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 195) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0},Color::RGB(200, 50, 100));
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 95) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_bottom = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 5) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_right = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 105) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));

        // buttons
        let key_state = KeyState { left: Note { state: false, active: true }, top: Note { state: false, active: true }, bottom: Note { state: false, active: true }, right: Note { state: false, active: true }};
        
        let mut bpm_bars:Vec<BeatLine> = vec![];

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
            maked_song: Song { name: "Test".to_owned(), id: Some(0), left_keys: vec![], up_keys: vec!(), bottom_keys: vec![], right_keys: vec![], end: 0, sync: Some(0), bpm: Some(0) },
            started_song: true,
            started_level: false,
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
            actual_button: 0,
            ui_texts,
            song_sync,
            bpm_timer: Instant::now(),
            bpm_bars
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App, fft_data: MutexGuard<'_, Vec<f32>>) {
        let texture_creator = app.canvas.texture_creator();
        match app_state.song_folder {
            Some(_) => {
                let delta_time = self.delta_time();
                let elapsed_time = self.start_time.elapsed() - self.paused_time;
                let mut milliseconds = 0;
                
                if app.paused && !self.end{ // pause state
                    milliseconds = 0;
                    if self.error == true{
                        self.ui_texts[3].render(&mut app.canvas, &texture_creator, &_font);

                        for (i, button) in self.error_elements.iter_mut().enumerate() {
                            if i == self.actual_button {
                                button.color = Color::RGB(0, 200, 0);
                            } else {
                                button.color = Color::RGB(100, 100, 100);
                            }
                            button.render(&mut app.canvas, &texture_creator, &_font);
                        }
                    } else {
                        // pause menu
                        self.ui_texts[0].render(&mut app.canvas, &texture_creator, &_font);

                        for (i, button) in self.pause_elements.iter_mut().enumerate() {
                            if i == self.actual_button {
                                button.color = Color::RGB(0, 200, 0);
                            } else {
                                button.color = Color::RGB(100, 100, 100);
                            }
                            button.render(&mut app.canvas, &texture_creator, &_font);
                        }
                    }
                } else {
                    if self.end == true {
                        app.canvas.set_draw_color(Color::RGBA(29, 91, 88, 100));
                        app.canvas.clear();

                        // i have to make sure so i can load the points of the player in a json
                            // first i have to check if the song haves an id that is not 0
                                // if id == 0 
                                    // 1. create id
                                    // 2. add it to the song
                                // then we take the id created and check if the id exists in the json of points
                                    // if it exists we change teh value of points
                                    // if not we add the new id and the points

                        self.ui_texts[1].text = Some("Congrats, you got ".to_owned() + &self.points.to_string().to_owned() + " points");
                        self.ui_texts[2].text = Some("Your max combo is ".to_owned() + &self.max_combo.to_string().to_owned() + " notes");
                        
                        self.ui_texts[1].render(&mut app.canvas, &texture_creator, &_font);
                        self.ui_texts[2].render(&mut app.canvas, &texture_creator, &_font);

                        for (i, button) in self.end_elements.iter_mut().enumerate() {
                            if i == self.actual_button {
                                button.color = Color::RGB(0, 200, 0);
                            } else {
                                button.color = Color::RGB(100, 100, 100);
                            }
                            button.render(&mut app.canvas, &texture_creator, &_font);
                        }
                    } else { // play state
                        match &app.textures.background {
                            Some(texture) => {
                                    app.canvas.copy(&texture, None, Some(Rect::new(0 as i32, 0 as i32, app.width as u32, app.height as u32)))
                                    .expect("Failed to copy texture into canvas");
                            },
                            None => {
                                app.canvas.set_draw_color(Color::RGBA(40, 40, 40, 100));
                                app.canvas.clear();
                            },
                        }
                        if app.visualizer_settings.bars {
                            visualize_space(app, &fft_data.to_vec());
                        }

                        if app.visualizer_settings.circle {
                            visualize_circle(app, &fft_data.to_vec());
                        }
                        
                        self.display_framerate(delta_time);

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

                        self.combo.text = Some(self.combo_val.to_string() + "x combo");
                        self.combo.render(&mut app.canvas, &texture_creator, _font);

                        match &self.song_keys {
                            Some(keys) => {
                                Self::handle_notes(self, milliseconds, delta_time, app)
                            },
                            None => {},
                        }

                        self.ui_elements[0].text = Some(self.points.to_string()); // point text
                        self.ui_elements[1].text = Some(format!("{}", milliseconds)); // timer

                        for button in &self.ui_elements {
                            button.render(&mut app.canvas, &texture_creator, &_font);
                        }

                        if !self.started_level {
                            self.started_level = true;
                            self.start_time = Instant::now();
                        }
                        
                        // audio loading and playing
                        if milliseconds >= 300 && self.started_song == true {
                            match app_state.state {
                                GameState::Playing => {
                                    self.started_song = false;
                                    match &self.song {
                                        Some(song) => {
                                            song.play(1).expect("The song didn't played");
                                            match &app.testing_song {
                                                Some(testing) => {
                                                    match mixer::Music::set_pos(((elapsed_time.as_millis() / 10) - app.paused_time / 10) as f64 + (testing.start_point) as f64 / 100.0) {
                                                        Ok(_) => {},
                                                        Err(_) => {
                                                            app.alert_message = String::from("the song position wasn't loaded correctly");
                                                            self.actual_button = 0;
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
                        
                        if milliseconds > self.song_end {
                            self.actual_button = 0;
                            self.end = true;
                            mixer::Music::pause();
                        }
                        app.canvas.set_draw_color(Color::RGB(235, 219, 178)); // it must be a Color::RGB() or other
                        app.canvas.fill_rect(Rect::new(0, (app.height - 5) as i32, ((app.width as f32 / self.song_end as f32) * milliseconds as f32) as u32, 5)).unwrap();
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
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[0]).unwrap() => {
                    if app.paused && self.error {
                        Self::reset(app, app_state);
                    } else if app.paused && !self.error {
                        Self::unpause(app);
                    } else if self.end {
                        Self::reset(app, app_state)
                    }
                    
                    if app.paused || self.end || self.error {
                        // check why i have this
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[1]).unwrap() => {
                    if self.actual_button > usize::MIN && (app.paused || self.end || self.error) {
                        self.actual_button -= 1;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[2]).unwrap() => {
                    if app.paused && !self.end {
                        if self.error {
                            if self.actual_button < self.error_elements.len() - 1 {
                                self.actual_button += 1;
                            }
                        } else {
                            if self.actual_button < self.pause_elements.len() - 1 {
                                self.actual_button += 1;
                            }
                        }
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[3]).unwrap() => {
                    if app.paused && self.error {
                        if self.actual_button == 0 {
                            Self::reset(app, app_state)
                        }
                    } else if app.paused && !self.error {
                        if self.actual_button == 0 {
                            Self::unpause(app);
                        } else if self.actual_button == 1 {
                            Self::reset(app, app_state);
                        }
                    } else if self.end {
                        Self::reset(app, app_state)
                    }
                },
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
        /* 
        match app.bpm {
            Some(bpm) => {
                if (self.bpm_timer.elapsed().as_millis() / 10) > 6000 / bpm as u128 && milliseconds > (300 - (app.coordination_data.base_time - self.song_sync)).try_into().unwrap() {
                    self.bpm_bars.push(BeatLine::new(app.width, app.height));
                    self.bpm_timer = Instant::now();
                }
            },
            None => {},
        }

        for bars in &mut self.bpm_bars {
            bars.render(app, delta_time);
            bars.game_object.y += app.coordination_data.key_speed * delta_time.as_secs_f32();
            bars.rect.y = bars.game_object.y as i32;
        }
        */

        if let Some(song_keys) = &mut self.song_keys {
            for key_index in [4,0,1,2,3] {
                
                let mut actual_key = match key_index {
                    0 => Some(&mut self.key_left),
                    1 => Some(&mut self.key_up),
                    2 => Some(&mut self.key_bottom),
                    3 => Some(&mut self.key_right),
                    _ => None
                };

                let mut remove: Vec<usize> = Vec::new(); // Collect indices of notes to remove

                for (i, note) in song_keys[key_index].iter_mut().enumerate() {
                    if note.game_object.y > app.width as f32 {
                        remove.push(i);
                    }

                    if note.mili - (app.coordination_data.base_time - self.song_sync) < milliseconds.try_into().unwrap() {
                        note.render(app);
                        note.update(delta_time, app.coordination_data.key_speed);

                        match actual_key {
                            Some(ref mut key_actual) => {
                                if (note.game_object.y + 50.0 > (self.canvas_height - 150) as f32) && (note.game_object.y < (self.canvas_height - 80) as f32) && key_actual.pressed && key_actual.timer_hold.elapsed().as_millis() / 10 < 10 {
                                    if note.game_object.active {
                                        if note.holding {

                                            key_actual.timer_hold = Instant::now();
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
                                    
                                            key_actual.state = 2;
                                        
                                } else if (note.game_object.y > (self.canvas_height - 80) as f32) && !note.muted && note.game_object.active {
                                    if !note.muted {
                                        self.combo_val = 0;
                                    }
                                    note.muted = true;                          
                                }
                            },
                            None => {},
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

fn visualize_space(app: &mut App, fft_data: &Vec<f32>) {
    let width = (app.width / NUM_BARS as u32) / 4 as u32;
        let total_width = width * NUM_BARS as u32;
        let remaining_space = app.width - total_width;
        let start_x = remaining_space / 2;
        let mut space = start_x as i32;

        for i in 0..NUM_BARS {
            let bar_height = fft_data[i] * 500.0; // Adjust the scale as needed
            let mid_up = app.height as i32 / 2 as i32 - bar_height as i32;
            let mid_down = app.height as i32 / 2 as i32;
            let bar_rect = Rect::new(space, mid_down, (width as f32 * 0.8) as u32 , bar_height as u32);
            let bar_rect_up = Rect::new(space, mid_up, (width as f32 * 0.8) as u32, bar_height as u32);
            app.canvas.set_draw_color(Color::RGB(60, 56, 54));
            app.canvas.fill_rect(bar_rect).unwrap();
            app.canvas.fill_rect(bar_rect_up).unwrap();
            space += width as i32;
        }
}

fn visualize_circle(app: &mut App, fft_data: &Vec<f32>) {
    let center_x = (app.width / 2) as i32;
    let center_y = (app.height / 2) as i32;
    
    let radius = fft_data[0] * 500.0;
    app.canvas.set_draw_color(Color::RGB(146, 131, 116));
    
    draw_circle(&mut app.canvas, center_x, center_y, radius as i32);
}

fn draw_circle(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32) {
    let mut x = 0;
    let mut y = radius;
    let mut decision = 3 - 2 * radius;

    while y >= x {
        // Draw points at all octants
        canvas.draw_point(Point::new(center_x + x, center_y + y)).unwrap(); // Octant 1
        canvas.draw_point(Point::new(center_x - x, center_y + y)).unwrap(); // Octant 2
        canvas.draw_point(Point::new(center_x + x, center_y - y)).unwrap(); // Octant 8
        canvas.draw_point(Point::new(center_x - x, center_y - y)).unwrap(); // Octant 7
        canvas.draw_point(Point::new(center_x + y, center_y + x)).unwrap(); // Octant 4
        canvas.draw_point(Point::new(center_x - y, center_y + x)).unwrap(); // Octant 3
        canvas.draw_point(Point::new(center_x + y, center_y - x)).unwrap(); // Octant 5
        canvas.draw_point(Point::new(center_x - y, center_y - x)).unwrap(); // Octant 6

        // Update x or y depending on decision parameter
        if decision <= 0 {
            decision += 4 * x + 6;
            x += 1;
        } else {
            decision += 4 * (x - y) + 10;
            x += 1;
            y -= 1;
        }
    }
}

#[derive(Clone, Copy)]
struct BeatLine {
    game_object: GameObject,
    rect: Rect
}

impl BeatLine {
    pub fn new(width: u32, _height: u32) -> Self {
        let game_object = GameObject { active: true, x: (width / 2 - (((width/2) - 200) / 2)) as f32, y: -100.0 - (5 / 2) as f32, width: (width/2) as f32  - 200.0, height: 5 as f32 };
        let bar_rect = Rect::new(game_object.x as i32, game_object.y as i32, game_object.width as u32, game_object.height as u32);

        BeatLine {
            rect: bar_rect,
            game_object
        }
    }

    pub fn render(mut self, app: &mut App, deltatime: Duration) {
        app.canvas.set_draw_color(Color::RGB(60, 56, 54));
        app.canvas.fill_rect(self.rect).unwrap();
    }
}
