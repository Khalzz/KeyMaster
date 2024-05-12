use std::fs;
use sdl2::{pixels::Color, ttf::Font, event::Event, keyboard::Keycode};
use crate::{app::{App, AppState, GameState, Testing}, game_object::GameObject, input::{button_module::{Button, TextAlign}, keybutton::{KeyButton, Note}, slider_module::Slider_input}, key::{GameKey, KeyFlag}, load_song::Song};

pub struct AddHolding {
    pub can_add: bool,
    pub add: bool
}

pub struct Selected {
    key: usize,
    flag: KeyFlag
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
    selected_object: Option<Selected>,
    buttons: Vec<Button>,
    changing_start: bool,
    add_key: bool,
    add_holding: AddHolding,
    start_point:  f64,
    scroll_slider: Slider_input,
    error: bool,
    end: u128
} 

impl GameLogic {
    // this is called once
    pub fn new(app: &mut App,  app_state: &mut AppState,  _font: &Font,) -> Self {
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

        let start_index = 0;
        let mut error = false;
        let mut end = 0;
        app.alert_message = String::from("");
        app.paused = false;
        let mut keys = vec![];

        match &app_state.song_folder {
            Some(folder) => {
                match Song::new(folder) {
                    Ok(song) => {
                        song_game = song.clone();
                        keys = song_game.clone().get_keys(app, true);
                        end = song_game.end.clone();
                    },
                    Err(_) => {
                        error = true;
                    },
                }
            },
            None => {
                error = true;
            },
        }

        let scroll_slider = Slider_input::new(
            app,
            GameObject {active: true, x: ((app.width/2) + 450) as f32, y: 0 as f32, width: 50.0, height: app.height as f32},
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            app.volume_percentage,
            true,
            None,
            true,
            20000.0
        );

        //let mut keys = ;

        let save = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 10.0, width: 100.0, height: 40.0}, Some(String::from("save")), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),None, TextAlign::Center);
        let play = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 60.0, width: 100.0, height: 40.0}, Some(String::from("play")), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),None, TextAlign::Center);
        
        let add_single_key = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 130.0, width: 100.0, height: 40.0}, Some(String::from("Add key")), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),None, TextAlign::Center);
        let add_holding_key = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 180.0, width: 100.0, height: 40.0}, Some(String::from("Add Hold")), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),None, TextAlign::Center);
        let time_position = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 300.0, width: 100.0, height: 40.0}, Some(String::from("000")), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),None, TextAlign::Center);
        let add_testing_start = Button::new(GameObject { active: true, x:(app.width - 110) as f32, y: 230.0, width: 100.0, height: 40.0}, Some(String::from("Add start")), Color::RGB(100, 100, 100), Color::WHITE, Color::RGB(0, 100, 0), Color::RGB(0, 0, 0),None, TextAlign::Center);

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
            song_game: Some(song_game),
            keys,
            start_index,
            ctrl: false,
            note_spaces_mod: 5.0,
            index_range: 200,
            selected_object: None,
            buttons: vec![save, play, add_single_key, add_holding_key, add_testing_start, time_position],
            changing_start: false,
            add_key: false,
            add_holding: AddHolding { can_add: false, add: false },
            start_point: 300.0,
            scroll_slider,
            error,
            end
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        let texture_creator = app.canvas.texture_creator();

        self.buttons[4].text = Some(self.start_index.to_string());
        match app_state.song_folder {
            Some(_) => {
                if self.error {
                    app.reseted = false;
                    app_state.state = GameState::Playing;
                } else {
                    app.canvas.set_draw_color(Color::RGBA(29, 91, 88, 100));
                    app.canvas.clear();
                    
                    for button in &self.buttons {
                        button.render(&mut app.canvas, &texture_creator, _font);
                    }
                    
                    

                    for list in [4,0,1,2,3] {
                        let mut note_spaces = 0.0;
                        let temp_list = self.start_index as usize ..(self.start_index as usize + self.index_range as usize);
                        if self.start_index < self.end - self.index_range as u128 {
                            for key in temp_list.into_iter() {
                                note_spaces += 0.70 * self.note_spaces_mod; 
                                match &self.selected_object {
                                    Some(selected) => {
                                        if key == selected.key {
                                            self.keys[list][key].color = Color::RGB(100, 100, 100);
                                        }
                                    }
                                    None => {}
                                }
                                
                                self.keys[list][key].render(app);
                                self.keys[list][key].game_object.y = app.height as f32 - (note_spaces);
                                note_spaces += 0.70 * self.note_spaces_mod;
                            }
                        } else {
                            self.start_index = self.end - self.index_range as u128 - 10
                        }

                        let mut key_buttons = [&self.key_left, &self.key_up, &self.key_right, &self.key_bottom];
                        for (i, button_key) in key_buttons.iter_mut().enumerate() {
                            button_key.render(app, i);
                        }
                        // self.scroll_slider.render(app, _font);
                    }
                }
    
                Self::event_handler(self, app_state, event_pump, app);
            },
            None => {},
        }
    }

    fn event_handler(&mut self, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
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
                            match selected.flag {
                                KeyFlag::Left => {
                                    println!("left");
                                    let empty_note = GameKey::new(GameObject {active: true, x: ((app.width/2) - 160) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as i128, None, None, false);
                                    delete_key(&selected, 0, empty_note, &mut self.keys);
                                },
                                KeyFlag::Up => {
                                    println!("up");
                                    let empty_note = GameKey::new(GameObject {active: true, x: ((app.width/2) - 60) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as i128, None, None, false);
                                    delete_key(&selected, 1, empty_note, &mut self.keys);
                                },
                                KeyFlag::Bottom => {
                                    let empty_note = GameKey::new(GameObject {active: true, x: ((app.width/2) + 40) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as i128, None, None, false);
                                    delete_key(&selected, 2, empty_note, &mut self.keys);
                                },
                                KeyFlag::Right => {
                                    let empty_note = GameKey::new(GameObject {active: true, x: ((app.width/2) + 140) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, 0 as i128, None, None, false);
                                    delete_key(&selected, 3, empty_note, &mut self.keys);
                                },
                                KeyFlag::Bpm => {},
                            }
                            self.selected_object = None;
                        },
                        None => {},
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    match &self.song_game {
                        Some(song_game) => {
                            if self.start_index < song_game.end - 150 {
                                self.start_index += 100;
                            }
                        }
                        None => {}
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if self.start_index > 100 {
                        self.start_index -= 100;
                    }
                }
                Event::MouseWheel { y, .. } => {
                    match &self.song_game {
                        Some(song_game) => {
                            if y == 1 && self.start_index < song_game.end - 200 {
                                self.start_index += 10;
                            } else if y == -1 && self.start_index > 10 {
                                self.start_index -= 10;
                            } else if y == -1 && self.start_index < 10 {
                                self.start_index = 0;
                            }
                        }
                        None => {}
                    }
                }
                Event::Quit { .. } => {
                    app_state.is_running = false;
                }
                _ => {}
            }

            if self.buttons[0].on_click(&event) { // save
                self.save(app_state)
            }

            if self.buttons[1].on_click(&event) { // play
                match &self.song_game {
                    Some(_song) => {
                        app.testing_song = Some(Testing{song: self.generate_array(), start_point: self.start_point - app.coordination_data.base_time as f64});
                    },
                    None => {},
                }
                app.reseted = false;
                app_state.state = GameState::Playing;
            }

            if self.buttons[2].on_click(&event) { // add key
                if self.add_key == false {
                    self.add_key = true;
                } else {
                    self.add_key = false;
                }
                self.buttons[2].toggle = Some(self.add_key);

            }

            if self.buttons[3].on_click(&event) { // play
                if self.changing_start == false {
                    self.buttons[3].text = Some("Del start".to_owned());
                    self.changing_start = true;
                } else {
                    self.buttons[3].text = Some("Add start".to_owned());
                    self.changing_start = false;
                }
            }


            for (i, list) in self.keys.iter_mut().enumerate() {
                if self.start_index < self.end - self.index_range as u128 {
                    for key in self.start_index as usize..self.start_index as usize + self.index_range as usize {
                        list[key].is_hover(&event);
                        if list[key].hover {
                            match list[key].flag {
                                Some(flag) => {
                                    match flag {
                                        KeyFlag::Bpm => {},
                                        _ => {
                                            list[key].color = Color::RGB(0, 50, 50);
                                        }
                                    }
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
                                                    list[key].color = Color::RGB(200, 200, 200);
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
                                    if key == self.start_point as usize {
                                        list[key].color = Color::RGB(200, 0, 0);
                                    } else {
                                        list[key].color = Color::RGB(0, 0, 0);
                                    }
                                },
                            }
                        }

                        if list[key].is_clicked(&event) {
                            match list[key].connected {
                                Some(con) => {
                                    println!("clicked: {}, connected to: {}", key, con)
                                },
                                None => {},
                            }

                            if self.add_key {
                                if i == 0 {
                                    list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) - 175) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as i128, Some(KeyFlag::Left), None, false);
                                } else if i == 1 {
                                    list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) - 75) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as i128, Some(KeyFlag::Up), None, false);
                                } else if i == 2 {
                                    list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) + 25) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as i128, Some(KeyFlag::Bottom), None, false);
                                } else if i == 3 {
                                    list[key] = GameKey::new(GameObject {active: true, x: ((app.width/2) + 125) as f32, y: 0.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), app.coordination_data.key_speed, 0 as i128, Some(KeyFlag::Right), None, false);
                                }
                            }

                            if self.changing_start == true {
                                self.start_point = key as f64;
                            } else {
                                match &self.selected_object {
                                    Some(selected) => {
                                        list.swap(selected.key, key);
                                        match list[key].connected {
                                            Some(con) => {
                                                list[con as usize].connected = Some(key as u128);
                                            },
                                            None => {},
                                        }
                                        self.selected_object = None;
                                    }
                                    None => {
                                        match &list[key].flag {
                                            Some(flag) => {
                                                match flag {
                                                    KeyFlag::Bpm => {},
                                                    _ => {
                                                        if !self.add_key {
                                                            self.selected_object = Some(Selected{ key, flag: flag.clone()});
                                                        }
                                                    }
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
                                match list[key].connected {
                                    Some(con_value) => {
                                        if key < con_value as usize {
                                            match flag {
                                                KeyFlag::Left => {
                                                    left_keys.push(Note { time: key as u128, holding: con_value as u128 - key as u128});
                                                },
                                                KeyFlag::Up => {
                                                    up_keys.push(Note { time: key as u128, holding: con_value - key as u128});
                                                },
                                                KeyFlag::Bottom => {
                                                    bottom_keys.push(Note { time: key as u128, holding: con_value - key as u128});
                                                },
                                                KeyFlag::Right => {
                                                    right_keys.push(Note { time: key as u128, holding: con_value - key as u128});
                                                },
                                                KeyFlag::Bpm => {},
                                            }
                                        }
                                    },
                                    None => {
                                        match flag {
                                            KeyFlag::Left => {
                                                left_keys.push(Note { time: key as u128, holding: 0});
                                            },
                                            KeyFlag::Up => {
                                                up_keys.push(Note { time: key as u128, holding: 0});
                                            },
                                            KeyFlag::Bottom => {
                                                bottom_keys.push(Note { time: key as u128, holding: 0});
                                            },
                                            KeyFlag::Right => {
                                                right_keys.push(Note { time: key as u128, holding: 0});
                                            },
                                            KeyFlag::Bpm => {},
                                        }
                                    },
                                }
                            },
                            None => {},
                        }
                    }
                }
                
                

                let edited_song = Song { name: song.name.clone(), id: song.id.clone(), left_keys, up_keys, bottom_keys, right_keys, end: song.end, sync: song.sync, bpm: song.bpm};
                return edited_song;
            },
            None => {
                return Song { name: " ".to_owned(), id: Some(0), left_keys: vec![], up_keys: vec![], bottom_keys: vec![], right_keys: vec![], end: 0, sync: Some(0), bpm: Some(0) };
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

fn delete_key(selected: &Selected, index: usize, empty_note: GameKey, keys: &mut Vec<Vec<GameKey>>) {
    match keys[index][selected.key].connected{
        Some(connected) => {
            if connected > selected.key as u128 {
                keys[index][connected as usize] = empty_note;
            }
        },
        None => {},
    }
    keys[index][selected.key] = empty_note;
}