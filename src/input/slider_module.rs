use sdl2::{render::{Canvas, TextureCreator, TextureQuery}, ttf::Font, video::{Window, WindowContext}, mouse::MouseButton, mixer};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::{game_object::GameObject, app::{App, self}};

use super::button_module::Button;

pub struct Slider_input {
    pub game_object: GameObject,
    pub color: Color,
    pub base_color: Color,
    pub hover_color: Color,
    pub clicked_color: Color,
    pub selected_amount: i32,
    pub hover: bool,
    pub clicked: bool,
    pub slider_position: i32,
    pub texts: [Button; 2],
    pub percentage: i32,
    pub end_percentage: i32,
}

impl Slider_input {
    pub fn new(app: &mut App, game_object: GameObject, color: Color, hover_color: Color, clicked_color: Color, percentage: i32) -> Self {
        let text = Button::new(
            GameObject {active: true, x:(game_object.x - 110.0) as f32, y: game_object.y - (50 / 2) as f32 + game_object.height / 2 as f32, width: 100.0, height: 50.0},
            String::from("Audio"),
            Color::RGB(0, 0, 0),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
        );

        let text2 = Button::new(
            GameObject {active: true, x:(game_object.x + 10.0 + game_object.width) as f32, y: game_object.y - (50 / 2) as f32 + game_object.height / 2 as f32, width: 100.0, height: 50.0},
            String::from("0%"),
            Color::RGB(0, 0, 0),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
        );

        let texts = [text, text2];
        let width = game_object.width;

        Slider_input {
            game_object,
            color,
            base_color: color,
            hover_color,
            clicked_color,
            hover: false,
            clicked: false,
            percentage,
            selected_amount: (width as i32 /100) * percentage,
            slider_position: (width as i32 /100) * percentage,
            end_percentage: percentage,
            texts
        }
    }

    pub fn render(&mut self, app: &mut App, _font: &Font) {
        if self.game_object.active == true {
            app.canvas.set_draw_color(self.base_color); // it must be a Color::RGB() or other
            app.canvas.fill_rect(Rect::new(self.game_object.x as i32, self.game_object.y as i32, self.game_object.width as u32, self.game_object.height as u32)).unwrap();
            app.canvas.set_draw_color(self.hover_color); // it must be a Color::RGB() or other
            app.canvas.fill_rect(Rect::new(self.game_object.x as i32, self.game_object.y as i32, self.selected_amount as u32, self.game_object.height as u32)).unwrap();

            for btn in 0..self.texts.len() {
                self.texts[btn].render(&mut app.canvas, &app.texture_creator, _font)
            }

            self.texts[1].text = self.end_percentage.to_string() + "%";
        }
    }

    pub fn is_hover(&mut self, event: &sdl2::event::Event, app: &mut App) {
        if self.game_object.active {
            match event { 
                sdl2::event::Event::MouseMotion {x, y, .. } => {

                    if (x > &(self.game_object.x as i32) && x < &(self.game_object.x as i32 + (self.game_object.width as i32))) && (y >= &(self.game_object.y as i32) && y <= &(self.game_object.y as i32 + (self.game_object.height as i32))) {
                        self.hover = true;
                        self.slider_position = *x - (app.width as i32/2 as i32 - self.game_object.width as i32/2 as i32);
                        self.percentage = ((self.slider_position as f32 / 200 as f32) * 100.0) as i32;
                    } else {
                        self.slider_position = self.selected_amount;
                        self.percentage = self.end_percentage;
                    }
                },
                sdl2::event::Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    if self.hover {
                        self.clicked = true;
                    }
                },
                sdl2::event::Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    if self.hover {
                        self.clicked = false;
                    }
                },
                _ => {} // in every other case we will do nothing    
            } 

            if self.hover && self.clicked {
                self.selected_amount = self.slider_position;
                self.end_percentage = self.percentage;
                //println!("{}", (128/self.end_percentage) * 100);
                if self.end_percentage < 1 {
                    self.end_percentage = 1;
                }
                mixer::Music::set_volume(((self.end_percentage as f32 / 100.0) * 128.0) as i32);
                
            }
        } else {
        }
    }
}