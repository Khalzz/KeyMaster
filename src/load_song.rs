use std::time::Duration;

use sdl2::{pixels::Color, rect::Rect};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{app::App, game_object::GameObject, input::keybutton::Note, key::{GameKey, KeyFlag}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    pub(crate) name: String,
    pub(crate) id: Option<i128>,
    pub(crate) left_keys:  Vec<Note>,
    pub(crate) up_keys: Vec<Note>,
    pub(crate) bottom_keys: Vec<Note>,
    pub(crate) right_keys: Vec<Note>,
    pub(crate) end: u128,
    pub(crate) sync: Option<i128>,
    pub(crate) bpm: Option<u128>,
}

// this struct loads the data from a json so is runned from the play.rs file
impl Song {
    pub fn new(folder: &String) -> Result<Song, Box<dyn std::error::Error>> {
        let mut song: Song = Song { name: "".to_owned(), id: Some(0), left_keys: vec![], up_keys: vec![], bottom_keys: vec![], right_keys: vec![], end: 0, sync: Some(0), bpm: Some(0) };
        match std::fs::read_to_string("songs/".to_owned() + &folder + "/data.json") {
            Ok(file_contents) => {
                let mut new_song: Song = serde_json::from_str(&file_contents)?;
                match new_song.id {
                    Some(_) => {},
                    None => new_song.id = Some(0),
                }
                song = Song { name: new_song.name, id: new_song.id, left_keys: new_song.left_keys, up_keys: new_song.up_keys, bottom_keys: new_song.bottom_keys, right_keys: new_song.right_keys, end: new_song.end, sync: new_song.sync, bpm: new_song.bpm};
            },
            Err(_) => {
                eprintln!("The song json didn't loaded right");
            },
        }
        Ok(song)
    }

    pub fn get_keys(self, app: &mut App, edit: bool) -> Vec<Vec<GameKey>> {
        let mut left_keys: Vec<GameKey> = Vec::new();
        let mut up_keys: Vec<GameKey> = Vec::new();
        let mut bottom_keys: Vec<GameKey> = Vec::new();
        let mut right_keys: Vec<GameKey> = Vec::new();

        let mut bpm_bars: Vec<GameKey> = vec![];

        
        let width = app.width;
        let key_speed = app.coordination_data.key_speed;
        
        if edit == true {
            for spaces in 0..self.end {
                left_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) - 160) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as i128, None, None, false));
                up_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) - 60) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as i128, None, None, false));
                bottom_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) + 40) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as i128, None, None, false));
                right_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) + 140) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as i128, None, None, false));
                bpm_bars.push(GameKey::new(GameObject { active: true, x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as i128, None, None, false));
            }

            Self::edit_list(self.left_keys.clone(),(width/2) - 175, (width/2) - 160, &mut left_keys, key_speed, KeyFlag::Left);
            Self::edit_list(self.up_keys.clone(),(width/2) - 75,(width/2) - 60,  &mut up_keys, key_speed, KeyFlag::Up);
            Self::edit_list(self.bottom_keys.clone(),(width/2) + 25,(width/2) + 40,  &mut bottom_keys, key_speed, KeyFlag::Bottom);
            Self::edit_list(self.right_keys.clone(),(width/2) + 125,(width/2) + 140,  &mut right_keys, key_speed, KeyFlag::Right);
            Self::bpm_list(self.end, self.right_keys.clone(),(width / 2 - (((width/2) - 200) / 2)) as u32,app.width,  &mut bpm_bars, key_speed, KeyFlag::Bpm, app, edit, self.bpm);
        } else {
            Self::play_list(self.left_keys.clone(),(width/2) - 175, (width/2) - 160, &mut left_keys, key_speed, KeyFlag::Left, app);
            Self::play_list(self.up_keys.clone(),(width/2) - 75,(width/2) - 60,  &mut up_keys, key_speed, KeyFlag::Up, app);
            Self::play_list(self.bottom_keys.clone(),(width/2) + 25,(width/2) + 40,  &mut bottom_keys, key_speed, KeyFlag::Bottom, app);
            Self::play_list(self.right_keys.clone(),(width/2) + 125,(width/2) + 140,  &mut right_keys, key_speed, KeyFlag::Right, app);
            Self::bpm_list(self.end, self.right_keys.clone(),(width / 2 - (((width/2) - 200) / 2)) as u32,app.width,  &mut bpm_bars, key_speed, KeyFlag::Bpm, app, edit, self.bpm);
        }

        return vec![left_keys, up_keys, bottom_keys, right_keys, bpm_bars]
    }

    pub fn play_list(self_list:Vec<Note>, x: u32, x2: u32, keys_list: &mut Vec<GameKey>, key_speed: f32, flag: KeyFlag, app: &mut App) {
        let color = Color::RGB(0, 0, 0);

        for values in self_list.clone().iter_mut() {
            if u128::from(values.time) > app.coordination_data.base_time.try_into().unwrap() {
                if values.holding > 0 {
                    for between_index in 0..values.holding {
                        let new_arrow: GameKey;
                        if between_index == 0 {
                            new_arrow = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, ((values.time as i128) + between_index as i128).try_into().unwrap(), Some(flag.clone()),None, false);
                        } else {
                            new_arrow = GameKey::new(GameObject {active: true, x: x2 as f32, y: -100.0, width: 20.0, height: 50.0}, color, key_speed, ((values.time as i128) + between_index as i128).try_into().unwrap(), Some(flag.clone()), Some((values.time as i128).try_into().unwrap()), true, );
                        }
                        keys_list.push(new_arrow);
                    }
                } else {
                    let new_arrow = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, values.time as i128, Some(flag.clone()), None, false);
                    keys_list.push(new_arrow);
                }
            }
        }
    }

    pub fn edit_list(self_list:Vec<Note>, x: u32, x2: u32, keys_list: &mut Vec<GameKey>, key_speed: f32, flag: KeyFlag) {
        for (i, _values) in keys_list.clone().iter().enumerate() {
            match Self::contains_note(i as u128, self_list.clone()) {
                Some(note) => {
                    if note.holding >= 50 {
                        keys_list[i] = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(200, 200, 0), key_speed, (note.time as i128).try_into().unwrap(), Some(flag.clone()),Some(i as u128 + note.holding), false);
                        keys_list[i + note.holding as usize] = GameKey::new(GameObject {active: true, x: x2 as f32, y: -100.0, width: 20.0, height: 6.0}, Color::RGB(0, 0, 200), key_speed, (note.time as i128) + note.holding as i128, Some(flag.clone()), Some(i as u128), false)
                    } else {
                        keys_list[i] = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, note.time as i128, Some(flag.clone()), None, false);
                    }
                },
                None => {},
            }
        }
    }

    
    pub fn bpm_list(end: u128, self_list:Vec<Note>, x: u32, x2: u32, keys_list: &mut Vec<GameKey>, key_speed: f32, flag: KeyFlag, app: &mut App, edit: bool, bpm: Option<u128>) {
        match bpm {
            Some(bpm) => {
                if bpm > 0 {
                    let mut value = 300;

                    if edit {
                        for (i, _values) in keys_list.clone().iter().enumerate() {
                            if value == i.try_into().unwrap() {
                                value += 6000 / bpm as u128;
                                keys_list[i] = GameKey::new(GameObject { active: true, x: (app.width / 2 - (((app.width/2) - 200) / 2)) as f32, y: -125.0, width: (app.width/2) as f32  - 200.0, height: 6 as f32 }, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, i as i128, Some(flag.clone()), None, false);
                            }
                        }
                    } else {
                        for moment in 0..end {
                            if moment == value {
                                keys_list.push(GameKey::new(GameObject { active: true, x: (app.width / 2 - (((app.width/2) - 500) / 2)) as f32, y: -100.0, width: (app.width/2) as f32  - 500.0, height: 6 as f32 }, Color::RGB(60, 56, 54), app.coordination_data.key_speed, moment as i128, Some(flag.clone()), None, false));
                                value += 6000 / bpm as u128;
                            }
                        }
                    }
                }
            }
            None => {},
        }
    }

    pub fn contains_note (space: u128, vector:Vec<Note>) -> Option<Note> {
        for note in vector {
            if note.time == space {
                return Some(note);
            }
        }
        return None;
    }
}

#[derive(Clone, Copy)]
struct BeatLine {
    game_object: GameObject,
    rect: Rect
}

impl BeatLine {
    pub fn new(width: u32, _height: u32) -> Self {
        let game_object = GameObject { active: true, x: (width / 2 - (((width/2) - 200) / 2)) as f32, y: -125.0, width: (width/2) as f32  - 200.0, height: _height as f32 };
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