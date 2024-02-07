use sdl2::pixels::Color;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{app::{self, App}, game_object::{self, GameObject}, input::keybutton::Note, key::GameKey};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    pub(crate) name: String,
    pub(crate) left_keys:  Vec<Note>,
    pub(crate) up_keys: Vec<Note>,
    pub(crate) bottom_keys: Vec<Note>,
    pub(crate) right_keys: Vec<Note>,
    pub(crate) end: u128
}

// this struct loads the data from a json so is runned from the play.rs file
impl Song {
    pub fn new(folder: &String) -> Result<Song, Box<dyn std::error::Error>> {
        let file_contents = std::fs::read_to_string("songs/".to_owned() + &folder + "/data.json")?;
    
        let new_song: Song = serde_json::from_str(&file_contents)?;
    
        Ok(Song {
            name: new_song.name,
            left_keys: new_song.left_keys,
            up_keys: new_song.up_keys,
            bottom_keys: new_song.bottom_keys,
            right_keys: new_song.right_keys,
            end: new_song.end,
        })
    }

    pub fn get_keys(mut self, app: &mut App, edit: bool) -> Vec<Vec<GameKey>> {
        let mut left_keys: Vec<GameKey> = Vec::new();
        let mut up_keys: Vec<GameKey> = Vec::new();
        let mut bottom_keys: Vec<GameKey> = Vec::new();
        let mut right_keys: Vec<GameKey> = Vec::new();

        
        let width = app.width;
        let coordination_start = app.coordination_data.base_time;
        let key_speed = app.coordination_data.key_speed;
        
        let left_game_object = GameObject {active: true, x: ((width/2) - 175) as f32, y: -100.0, width: 50.0, height: 50.0};
        let up_game_object = GameObject {active: true, x: ((width/2) - 75) as f32, y: -100.0, width: 50.0, height: 50.0};
        let bottom_game_object = GameObject {active: true, x: ((width/2) + 25) as f32, y: -100.0, width: 50.0, height: 50.0};
        let right_game_object = GameObject {active: true, x: ((width/2) + 125) as f32, y: -100.0, width: 50.0, height: 50.0};
        
        if edit == true {
            for spaces in 0..self.end {
                left_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) - 160) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as u128, None, None));
                up_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) - 60) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as u128, None, None));
                bottom_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) + 40) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as u128, None, None));
                right_keys.push(GameKey::new(GameObject {active: true, x: ((width/2) + 140) as f32, y: 0.0, width: 20.0, height: 6.0}, Color::RGBA(0, 0, 0,0), app.coordination_data.key_speed, spaces as u128, None, None));
            }

            Self::edit_list(self.left_keys.clone(),((width/2) - 175), ((width/2) - 160), &mut left_keys, width, coordination_start, key_speed, "Left".to_owned());
            Self::edit_list(self.up_keys.clone(),((width/2) - 75),((width/2) - 60),  &mut up_keys, width, coordination_start, key_speed, "Up".to_owned());
            Self::edit_list(self.bottom_keys.clone(),((width/2) + 25),((width/2) + 40),  &mut bottom_keys, width, coordination_start, key_speed, "Bottom".to_owned());
            Self::edit_list(self.right_keys.clone(),((width/2) + 125),((width/2) + 140),  &mut right_keys, width, coordination_start, key_speed, "Right".to_owned());
        } else {
            Self::play_list(self.left_keys.clone(),((width/2) - 175), ((width/2) - 160), &mut left_keys, width, coordination_start, key_speed, "Left".to_owned());
            Self::play_list(self.up_keys.clone(),((width/2) - 75),((width/2) - 60),  &mut up_keys, width, coordination_start, key_speed, "Up".to_owned());
            Self::play_list(self.bottom_keys.clone(),((width/2) + 25),((width/2) + 40),  &mut bottom_keys, width, coordination_start, key_speed, "Bottom".to_owned());
            Self::play_list(self.right_keys.clone(),((width/2) + 125),((width/2) + 140),  &mut right_keys, width, coordination_start, key_speed, "Right".to_owned());
        }

        return vec![left_keys, up_keys, bottom_keys, right_keys]
    }

    pub fn play_list(self_list:Vec<Note>, x: u32, x2: u32, keys_list: &mut Vec<GameKey>, width: u32, coordination_start: u128, key_speed: f32, flag: String) {
        for values in self_list.clone().iter_mut() {
            if u128::from(values.time) > coordination_start {
                if values.holding >= 50 {
                    for between_index in 0..values.holding {
                        let new_arrow: GameKey;
                        if between_index == 0 {
                            new_arrow = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, (values.time - coordination_start) + between_index, Some(flag.clone()),None);
                        } else {
                            new_arrow = GameKey::new(GameObject {active: true, x: x2 as f32, y: -100.0, width: 20.0, height: 50.0}, Color::RGB(0, 200, 200), key_speed, (values.time - coordination_start) + between_index, Some(flag.clone()),None);
                        }
                        keys_list.push(new_arrow);
                    }
                } else {
                    let new_arrow = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, values.time - coordination_start, Some(flag.clone()), None);
                    keys_list.push(new_arrow);
                }
            }
        }
    }

    pub fn edit_list(self_list:Vec<Note>, x: u32, x2: u32, keys_list: &mut Vec<GameKey>, width: u32, coordination_start: u128, key_speed: f32, flag: String) {
        for (i, values) in keys_list.clone().iter().enumerate() {
            match Self::contains_note(i as u128, self_list.clone()) {
                Some(note) => {
                    if note.holding >= 50 {
                        keys_list[i] = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(200, 200, 0), key_speed, (note.time - coordination_start), Some(flag.clone()),Some(i as u128 + note.holding));
                        keys_list[i + note.holding as usize] = GameKey::new(GameObject {active: true, x: x2 as f32, y: -100.0, width: 20.0, height: 6.0}, Color::RGB(0, 0, 200), key_speed, (note.time - coordination_start) + note.holding, Some(flag.clone()), Some(i as u128))
                    } else {
                        keys_list[i] = GameKey::new(GameObject {active: true, x: x as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, note.time - coordination_start, None, None);
                    }
                },
                None => {},
            }
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