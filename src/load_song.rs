use sdl2::pixels::Color;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{key::GameKey, game_object::GameObject, app::{self, App}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    pub(crate) name: String,
    pub(crate) left_keys:  Vec<u64>,
    pub(crate) up_keys: Vec<u64>,
    pub(crate) bottom_keys: Vec<u64>,
    pub(crate) right_keys: Vec<u64>,
    pub(crate) end: u128
}

// this struct loads the data from a json so is runned from the play.rs file
impl Song {
    pub fn new(folder: &String) -> Song {
        let file_contents = std::fs::read_to_string("songs/".to_owned() + &folder + "/data.json").expect("Failed to read file");
        let new_song : Song = serde_json::from_str(&file_contents).expect("Failed to parse JSON");

        Song { 
            name: new_song.name,
            left_keys: new_song.left_keys, 
            up_keys: new_song.up_keys, 
            bottom_keys: new_song.bottom_keys, 
            right_keys: new_song.right_keys,
            end: new_song.end
        }
    }

    pub fn get_keys(mut self, width: &mut u32, coordination_start: &u128, key_speed: f32) -> Vec<Vec<GameKey>> {
        let mut left_keys: Vec<GameKey> = Vec::new();
        let mut up_keys: Vec<GameKey> = Vec::new();
        let mut bottom_keys: Vec<GameKey> = Vec::new();
        let mut right_keys: Vec<GameKey> = Vec::new();

        for values in  self.left_keys.iter_mut() {
            if u128::from(*values) > *coordination_start {
                let new_arrow = GameKey::new(GameObject {active: true, x: ((*width/2) - 175) as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, *values as u128 - coordination_start, None);
                left_keys.push(new_arrow);
            }
        }

        for values in  self.up_keys.iter_mut() {
            if u128::from(*values) > *coordination_start {
                let new_arrow = GameKey::new(GameObject {active: true, x: ((*width/2) - 75) as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, *values as u128 - coordination_start, None);
                up_keys.push(new_arrow);
            }
        }

        for values in  self.bottom_keys.iter_mut() {
            if u128::from(*values) > *coordination_start {                
                let new_arrow = GameKey::new(GameObject {active: true, x: ((*width/2) + 25) as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, *values as u128 - coordination_start, None);
                bottom_keys.push(new_arrow);
            }
        }

        for values in  self.right_keys.iter_mut() {
            if u128::from(*values) > *coordination_start {                                
                let new_arrow = GameKey::new(GameObject {active: true, x: ((*width/2) + 125) as f32, y: -100.0, width: 50.0, height: 50.0}, Color::RGB(0, 200, 0), key_speed, *values as u128 - coordination_start, None);
                right_keys.push(new_arrow);
            }
        }

        return vec![left_keys, up_keys, bottom_keys, right_keys];
    }
}