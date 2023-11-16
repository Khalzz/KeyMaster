use std::{time::{Duration, Instant}, fs};
use sdl2::{render::{Canvas, TextureCreator}, video::{Window, WindowContext}, pixels::Color, ttf::Font, event::Event, keyboard::Keycode, image::LoadTexture, mixer::{self, Music}, mouse::MouseWheelDirection};
use crate::{key::GameKey, app::{App, AppState, GameState, self, Testing}, game_object::GameObject, input::keybutton::{KeyButton}, input::button_module::Button, load_song::Song};

pub struct KeyState {
    pub left: bool,
    pub top: bool,
    pub bottom: bool,
    pub right: bool
}

pub struct selected {
    key: usize,
    flag: String
}

pub struct GameLogic { // here we define the data we use on our script
    key_left: KeyButton,
    key_up: KeyButton,
    key_bottom: KeyButton,
    key_right: KeyButton,
    song_game: Option<Song>,
    keys: Vec<Vec<GameKey>>,
    start_index: u128,
    ctrl: bool,
    note_spaces_mod: f32,
    index_range: u64,
    selected_object: Option<selected>,
    buttons: Vec<Button>,
    changing_start: bool,
    add_key: bool,
    start_point:  f64
} 

impl GameLogic {
    // this is called once
    pub fn new(app: &mut App,  app_state: &mut AppState,  _font: &Font,) -> Self {
        let mut song_game = None;
        match &app_state.song_folder {
            Some(folder) => {
                song_game = Some(Song::new(folder));
            },
            None => {},
        }

        let mut left_keys = vec![];
        let mut up_keys = vec![];
        let mut bottom_keys = vec![];
        let mut right_keys = vec![];
        let start_index = 0;

        match &song_game {
            Some(song_data) => {
                for spaces in start_index..song_data.end {
                    if song_data.left_keys.contains(&(spaces as u64)) {
                        left_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) - 175) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Left".to_owned())));
                    } else {
                        left_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) - 160) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None));
                    } 
                    if song_data.up_keys.contains(&(spaces as u64)) {
                        up_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) - 75) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Up".to_owned())));
                    } else {
                        up_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) - 60) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None));
                    }
                    if song_data.bottom_keys.contains(&(spaces as u64)) {
                        bottom_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) + 25) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Bottom".to_owned())));
                    } else {
                        bottom_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) + 40) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None));
                    }
                    if song_data.right_keys.contains(&(spaces as u64)) {
                        right_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) + 125) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Right".to_owned())));
                    } else {
                        right_keys.push(GameKey::new(GameObject {active: true, x: ((app.width/2) + 140) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None));
                    }
                }
            },
            None => {},
        }

        let mut keys = vec![left_keys, up_keys, bottom_keys, right_keys];

        let save = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 10.0, width: 100.0, height: 40.0}, String::from("save"), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),);
        let play = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 60.0, width: 100.0, height: 40.0}, String::from("play"), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),);
        
        let add_single_key = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 130.0, width: 100.0, height: 40.0}, String::from("Add key"), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),);
        let add_testing_start = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 180.0, width: 100.0, height: 40.0}, String::from("Add start"), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),);

        // controlers 
        let key_left = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 195) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0},Color::RGB(200, 50, 100));
        let key_up = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) - 95) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_bottom = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 5) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        let key_right = KeyButton::new(app, GameObject {active: true, x: ((app.width/2) + 105) as f32, y: app.height as f32 - 170.0, width: 90.0, height: 90.0}, Color::RGB(200, 50, 100));
        
        Self {
            key_left,
            key_up,
            key_bottom,
            key_right,
            song_game,
            keys,
            start_index,
            ctrl: false,
            note_spaces_mod: 5.0,
            index_range: 1200,
            selected_object: None,
            buttons: vec![save, play, add_single_key, add_testing_start],
            changing_start: false,
            add_key: false,
            start_point: 300.0
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        match app_state.song_folder {
            Some(_) => {
                app.canvas.set_draw_color(Color::RGBA(29, 91, 88, 100));
                app.canvas.clear();

                for button in &self.buttons {
                    button.render(&mut app.canvas, &app.texture_creator, _font);
                }

                match &self.song_game {
                    Some(song) => {
                        let mut scroller_space = 0.0;

                        for element in 0..song.end {
                            let mut space = GameKey::new(GameObject {active: true, x: ((app.width/2) + 250) as f32, y: app.height as f32 - scroller_space, width: 5.0, height: 0.2}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Right".to_owned()));
                            if element > self.start_index && element < self.start_index + self.index_range as u128 {
                                space.color = Color::RGB(90, 90, 90);
                            }
                            space.render(&mut app.canvas);
                            scroller_space += 0.2;
                        }
                    },
                    None => {},
                }

                for list in &mut self.keys {
                    let mut note_spaces = 0.0;
                    let temp_list = self.start_index as usize ..(self.start_index as usize + self.index_range as usize);
                    
                    for key in temp_list.into_iter() {
                        note_spaces += 0.70 * self.note_spaces_mod; 

                        match &self.selected_object {
                            Some(selected) => {
                                if key == selected.key {
                                    list[key].color = Color::RGB(100, 100, 100);
                                }
                            }
                            None => {}
                        }

                        list[key].render(&mut app.canvas);
                        list[key].game_object.y = app.height as f32 - (note_spaces);
                        note_spaces += 0.70 * self.note_spaces_mod;
                    }
                }
    
                let mut key_buttons = [&self.key_left, &self.key_up, &self.key_right, &self.key_bottom];
                for button_key in key_buttons.iter_mut() {
                    button_key.render(Some("assets/sprites/WhiteKey-Sheet.png"), app);
                }

                Self::event_handler(self, app_state, event_pump, app);
                app.canvas.present();
            },
            None => {},
        }
    }

    fn event_handler(&mut self, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        let mut scroll_up = false;
    
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    app.testing_song = None;
                    app_state.state = GameState::SelectingSong;
                }
                Event::KeyDown { keycode: Some(Keycode::LCtrl), .. } => {
                    self.ctrl = true;
                }
                Event::KeyUp { keycode: Some(Keycode::LCtrl), .. } => {
                    self.ctrl = false;
                }
                Event::KeyDown { keycode: Some(Keycode::Delete), .. } => {                    
                    match &self.selected_object {
                        Some(selected) => {
                            if selected.flag == "Left" {
                                self.keys[0][selected.key] = GameKey::new(GameObject {active: true, x: ((app.width/2) - 160) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None);
                            } else if selected.flag == "Up" {
                                self.keys[1][selected.key] = GameKey::new(GameObject {active: true, x: ((app.width/2) - 60) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None);
                            } else if selected.flag == "Bottom" {
                                self.keys[2][selected.key] = GameKey::new(GameObject {active: true, x: ((app.width/2) + 40) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None);
                            } else if selected.flag == "Right" {
                                self.keys[3][selected.key] = GameKey::new(GameObject {active: true, x: ((app.width/2) + 140) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as u128, None);
                            }
                            self.selected_object = None;
                        },
                        None => {},
                    }
                }
                Event::Quit { .. } => {
                    app_state.is_running = false;
                }
                Event::MouseWheel { y, .. } => {
                    match &self.song_game {
                        Some(song_game) => {
                            if !self.ctrl {
                                if y == 1 && self.start_index < song_game.end - 150 {
                                    self.start_index += 10;
                                } else if y == -1 && self.start_index > 0 {
                                    self.start_index -= 10;
                                }
                            } else {
                                if y == 1 {
                                    self.note_spaces_mod += 0.1;
                                } else if y == -1 && (self.note_spaces_mod - 0.1) > 0.0 {
                                    self.note_spaces_mod -= 0.1;
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => {}
            }

            if self.buttons[0].on_click(&event) { // save
                self.save(app_state)
            }

            if self.buttons[1].on_click(&event) { // play
                match &self.song_game {
                    Some(song) => {
                        app.testing_song = Some(Testing{song: self.generate_array(), start_point: self.start_point});
                    },
                    None => {},
                }
                app.reseted = false;
                app_state.state = GameState::Playing;
            }

            if self.buttons[2].on_click(&event) { // play
                if self.add_key == false {
                    self.add_key = true;
                } else {
                    self.add_key = false;
                }
            }

            if self.buttons[3].on_click(&event) { // play
                if self.changing_start == false {
                    self.buttons[3].text = "Del start".to_owned();
                    self.changing_start = true;
                } else {
                    self.buttons[3].text = "Add start".to_owned();
                    self.changing_start = false;
                }
            }

            for (i, list) in self.keys.iter_mut().enumerate() {
                for key in self.start_index as usize..self.start_index as usize + self.index_range as usize {
                    list[key].is_hover(&event);
                    if list[key].hover {
                        match list[key].flag {
                            Some(_) => {
                                list[key].color = Color::RGB(0, 50, 50);
                            },
                            None => {
                                if self.changing_start == true {
                                    list[key].color = Color::RGB(200, 0, 100);
                                } else {
                                    match &self.selected_object {
                                        Some(_) => {
                                            list[key].color = Color::RGB(100, 0, 0);
                                        }
                                        None => {
                                            list[key].color = Color::RGB(0, 0, 0);
                                        }
                                    }
                                }
                            },
                        }
                    } else {
                        match list[key].flag {
                            Some(_) => {
                                list[key].color = Color::RGB(0, 200, 0);
                            },
                            None => {
                                list[key].color = Color::RGB(0, 0, 0);
                            },
                        }
                    }

                    if list[key].is_clicked(&event) {
                        if self.add_key {
                            if i == 0 {
                                list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) - 175) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Left".to_owned()));
                            } else if i == 1 {
                                list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) - 75) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Up".to_owned()));
                            } else if i == 2 {
                                list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) + 25) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Bottom".to_owned()));
                            } else if i == 3 {
                                list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) + 125) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as u128, Some("Right".to_owned()));
                            }
                        }

                        if self.changing_start == true {
                            self.start_point = key as f64 / 100.0;
                        } else {
                            match &self.selected_object {
                                Some(selected) => {
                                    list.swap(selected.key, key);
                                    self.selected_object = None;
                                }
                                None => {
                                    match &list[key].flag {
                                        Some(flag) => {
                                            if !self.add_key {
                                                self.selected_object = Some(selected{ key, flag: flag.clone()});
                                            }
                                        },
                                        None => {},
                                    }

                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn generate_array(&mut self) -> Song {
        match &self.song_game {
            Some(song) => {
                let mut left_keys = vec![];
                let mut up_keys = vec![];
                let mut bottom_keys = vec![];
                let mut right_keys = vec![];
                
                for list in &self.keys {
                    for key in 0..song.end as usize {
                        match &list[key].flag {
                            Some(flag) => {
                                // if its Left
                                if flag == "Left" {
                                    left_keys.push(key as u64);
                                } else if flag == "Up" {
                                    up_keys.push(key as u64);
                                } else if flag == "Bottom" {
                                    bottom_keys.push(key as u64);
                                } else if flag == "Right" {
                                    right_keys.push(key as u64);
                                }
                            },
                            None => {},
                        }
                    }
                }
                
                let edited_song = Song { name: song.name.clone(), left_keys, up_keys, bottom_keys, right_keys, end: song.end };
                return edited_song;
            },
            None => {
                return Song { name: " ".to_owned(), left_keys: vec![], up_keys: vec![], bottom_keys: vec![], right_keys: vec![], end: 0 };
            },
        }
    }

    fn save(&mut self, app_state: &mut AppState) {
                let edited_song = self.generate_array();
                save_json(&edited_song, app_state).err();
    }
}

fn save_json(song: &Song, app_state: &mut AppState) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string(song)?;
    match &app_state.song_folder {
        Some(folder) => {
            println!("{}", folder);
            fs::write("songs/".to_owned() + folder + "/data.json", json_string.to_string())?;
        },
        None => {},
    }
    Ok(())
}