use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use serde_json;
use std::fs;

use crate::app::App;
use crate::game_object::GameObject;
use crate::load_song::Song;

pub struct KeyButton {
    pub game_object: GameObject,
    pub color: Color,
    pub pressed: bool,
    pub repeat: bool,
    pub left_keys: Vec<u64>,
    pub right_keys: Vec<u64>,
    pub up_keys: Vec<u64>,
    pub down_keys: Vec<u64>,
    pub image_array: Vec<Rect>,
    pub state: usize
}

impl KeyButton {
    pub fn new(app: &mut App, game_object: GameObject, color: Color) -> KeyButton {
        // add here the texture initialization
        let image = app.texture_creator.load_texture("assets/sprites/WhiteKey-Sheet.png");
        let image_array = generate_sprite_array(1, 3, 32, 32);

        KeyButton {
            game_object,
            color: color,
            pressed: false,
            repeat: true,
            left_keys: vec![],
            right_keys: vec![],
            up_keys: vec![],
            down_keys: vec![],
            image_array,
            state: 0
        }
    }

    pub fn render(&self, image: Option<&str>, app: &mut App) {
        match image {
            Some(image) => {
                if self.game_object.active == true {
                    let image_texture = app.texture_creator.load_texture("assets/sprites/WhiteKey-Sheet.png").expect("Failed to load image texture");
                    app.canvas.copy(&image_texture, self.image_array[self.state], Some(Rect::new(self.game_object.x as i32, self.game_object.y as i32, self.game_object.width as u32, self.game_object.height as u32)))
                    .expect("Failed to copy texture into canvas");
                } else {
                    return
                }
            },
            None => {
                if self.game_object.active == true {
                    app.canvas.set_draw_color(self.color); // it must be a Color::RGB() or other
                    app.canvas.fill_rect(Rect::new(self.game_object.x as i32, self.game_object.y as i32, self.game_object.width as u32, self.game_object.height as u32)).unwrap();
                } else {
                    return
                }
            },
        }
    }

    pub fn update(&mut self,song: &mut Song, milliseconds: u128, event: &sdl2::event::Event, key: sdl2::keyboard::Keycode, play_keys: &mut [Keycode; 4]) -> bool {
        if self.game_object.active {
            match event {
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if *key_value == key => {
                    if self.repeat == true {

                        // song generation
                        if key == play_keys[0] {
                            self.left_keys.push(milliseconds as u64);
                        }
                        if key == play_keys[3] {
                            self.right_keys.push(milliseconds as u64);
                        }
                        if key == play_keys[1] {
                            self.up_keys.push(milliseconds as u64);
                        }
                        if key == play_keys[2] {
                            self.down_keys.push(milliseconds as u64);
                        }

                        self.repeat = false;
                        self.pressed = true;
                        self.state = 1;
                    }
                },
                sdl2::event::Event::KeyUp { keycode: Some(key_value), .. } if *key_value == key => {
                    if self.repeat == false {
                        self.repeat = true;
                        self.pressed = false;
                        self.state = 0;

                    }
                },
                // song generation (saving)
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    song.end = milliseconds;

                    if key == play_keys[0] {
                        song.left_keys = self.left_keys.clone();
                    }
                    if key == play_keys[3] {
                        song.right_keys = self.right_keys.clone();
                    }
                    if key == play_keys[1] {
                        song.up_keys = self.up_keys.clone();
                    }
                    if key == play_keys[2] {
                        song.bottom_keys = self.down_keys.clone();
                    }

                    if let Err(err) = save_json(&song) {
                        println!("Error saving JSON: {}", err);
                    }
                },
                _ => {}
            } 
        } else {
            self.pressed = false;
        }
        return self.pressed;
    }

}

fn save_json(song: &Song) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string(song)?;
    fs::write("data.json", json_string.to_string())?;
    
    Ok(())
}

fn generate_sprite_array(yamount: i32, xamount: i32, ysize: i32, xsize: i32) -> Vec<Rect> {
    let mut yam = 0;
    let mut xam = 0;
    let mut rects: Vec<Rect> = vec![];

    for row in 0..yamount {
        for column in 0..xamount {
            rects.push(Rect::new(xam, yam, xsize as u32, ysize as u32));
            xam += xsize;
        }
        yam += xsize;
    }

    return rects;
}