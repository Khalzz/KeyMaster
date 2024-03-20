use std::time::Duration;

use sdl2::image::LoadTexture;
use sdl2::mouse::MouseButton;
use sdl2::render::Texture;
use sdl2::sys::SDL_Texture;
use sdl2::{render::Canvas, video::Window, rect::Rect};
use sdl2::pixels::Color;

use std::collections::HashMap;

use crate::app::{self, App};
use crate::game_object::GameObject;
use crate::input::keybutton;

#[derive(Clone, Copy)]
pub enum KeyFlag {
    Left,
    Up,
    Bottom,
    Right
}

#[derive(Clone, Copy)]
pub struct GameKey {
    pub game_object: GameObject,
    pub color: Color,
    pub speed: f32,
    pub mili: i128,
    pub hover: bool,
    pub holding: bool,
    pub flag: Option<KeyFlag>,
    pub connected: Option<u128>,
    pub muted: bool,
}

impl GameKey {
    pub fn new(game_object: GameObject, color: Color, speed: f32, mili: i128, flag: Option<KeyFlag>, connected: Option<u128>, holding: bool) -> GameKey {
        GameKey {
            game_object,
            color: color,
            speed,
            mili,
            hover: false,
            holding,
            flag,
            connected,
            muted: false,
        }
    }

    pub fn render(&self, app: &mut App) {
        let mut note_texture = &app.textures.red_note;
        let mut hold_texture = &app.textures.red_hold;
        let mut end_texture = &app.textures.red_note;

        if self.game_object.active == true {
            match &self.flag {
                Some(flag) => {
                    match flag {
                        KeyFlag::Left => {
                            note_texture = &app.textures.red_note;
                            hold_texture = &app.textures.red_hold;
                        },
                        KeyFlag::Up => {
                            note_texture = &app.textures.yellow_note;
                            hold_texture = &app.textures.yellow_hold;
                        },
                        KeyFlag::Bottom => {
                            note_texture = &app.textures.blue_note;
                            hold_texture = &app.textures.blue_hold;
                        },
                        KeyFlag::Right => {
                            note_texture = &app.textures.purple_note;
                            hold_texture = &app.textures.purple_hold;
                        },
                    }
                    
                    if self.muted {
                        note_texture = &app.textures.mute_note;
                        hold_texture = &app.textures.mute_hold;
                    }

                    if self.holding {
                        end_texture = hold_texture;
                    } else {
                        end_texture = note_texture;
                    }
                },
                None => {
                    end_texture = &None;
                },
            }

            match end_texture {
                Some(texture) => {
                    app.canvas.copy(texture, None, Some(Rect::new(self.game_object.x as i32, self.game_object.y as i32, self.game_object.width as u32, self.game_object.height as u32)));
                    unsafe {
                        let raw_texture_ptr = texture as *const sdl2::render::Texture as *mut SDL_Texture;
                        sdl2::sys::SDL_DestroyTexture(raw_texture_ptr);
                    }
                },
                None => {
                    app.canvas.set_draw_color(self.color); // it must be a Color::RGB() or other
                    app.canvas.fill_rect(Rect::new(self.game_object.x as i32, self.game_object.y as i32, self.game_object.width as u32, self.game_object.height as u32)).unwrap();
                },
            }
        } else {
            return
        }
    }

    pub fn update(&mut self, deltatime: Duration, key_speed: f32) {
        // this is a key for a rythm game so its gonna move down :b
        self.game_object.y += key_speed * deltatime.as_secs_f32();
    }

    pub fn is_hover(&mut self, event: &sdl2::event::Event) {
        if self.game_object.active {
            match event { 
                sdl2::event::Event::MouseMotion {x, y, .. } => {
                    if (x > &(self.game_object.x as i32) && x < &(self.game_object.x as i32 + (self.game_object.width as i32))) && (y >= &(self.game_object.y as i32) && y <= &(self.game_object.y as i32 + (self.game_object.height as i32))) {
                        self.hover = true;
                    } else {
                        self.hover = false;
                    }
                },
                _ => {} // in every other case we will do nothing
            } 
        } else {
            self.hover = false;
        }
    }

    pub fn is_clicked(&mut self, event: &sdl2::event::Event) -> bool {
        self.is_hover(event);
        if self.game_object.active {
            match event { 
                sdl2::event::Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    if self.hover {
                        self.holding = true;
                    }
                },sdl2::event::Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    self.holding = false;
                },
                _ => {} // in every other case we will do nothing
            }   
            return self.holding;   
        } else {
            return false;
        }
    }
}