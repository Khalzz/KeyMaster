use std::{fs, path::Path};

use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::{Canvas, Texture, TextureCreator}, sys::{KeyCode, SDL_Texture}, ttf::Font, video::{Window, WindowContext}};
use crate::{ app::{App, AppState, GameState}, game_object::GameObject, input::button_module::{Button, TextAlign}};

pub struct SongFile {
    button: Button,
    img_texture: Option<Texture>
}

pub struct GameLogic<> { // here we define the data we use on our script
    btn_list: Vec<SongFile>,
    loading_text: Button,
    song_img: Button,
    actual_button: usize
}

impl GameLogic<> {
    // this is called once
    pub fn new(mut app: &mut App) -> Self {
        // read every file in ./song
        let songs = Self::load_songs("./songs".to_owned(), &mut app);
        let loading_text = Button::new( GameObject { active: true, x: ((app.width / 2) - ((app.width - 20) / 2)) as f32, y: (app.height as f32 - 60.0), width: (app.width as f32 - 20.0), height: 50.0}, Some("Loading".to_owned()), Color::RGB(28, 29, 37), Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0), None, TextAlign::Center);

        let song_img = Button::new( GameObject { active: true, x: (app.width as f32 - 600.0) / 2 as f32, y: (app.height as f32 / 2.0) - 300.0 / 2.0, width: 300.0, height: 300.0}, Some("No Cover".to_owned()), Color::RGB(28, 29, 37), Color::WHITE, Color::RGB(0, 200, 0), Color::RGB(0, 0, 0), None, TextAlign::Center);

        Self {
            btn_list: songs,
            loading_text,
            song_img,
            actual_button: 0
        }
    }

    // this is called every frame
    pub fn update(&mut self, _font: &Font, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {
        let texture_creator = app.canvas.texture_creator();
        for btn in 0..self.btn_list.len() {
            self.btn_list[btn].button.render(&mut app.canvas, &texture_creator, _font);
        }

        match &self.btn_list[self.actual_button].img_texture {
            Some(texture) => {
                    app.canvas.copy(&texture, None, Some(Rect::new((app.width as f32 - 600.0) as i32 / 2 as i32, ((app.height as f32 / 2.0) - 150.0) as i32, 300, 300))).unwrap();
                    unsafe {
                        let raw_texture_ptr = texture as *const sdl2::render::Texture as *mut SDL_Texture;
                        sdl2::sys::SDL_DestroyTexture(raw_texture_ptr);
                    }
            },
            None => {
                self.song_img.render(&mut app.canvas, &texture_creator, _font);
            },
        }

        for (i, btn) in self.btn_list.iter_mut().enumerate() {
            // buscamos en la lista los valores mayores y menores a este y en base a eso organizamos los elementos.
            
            if i == self.actual_button {
                btn.button.color = Color::RGB(0, 200, 0);
                btn.button.game_object.y = app.height as f32 / 2.0 - (btn.button.game_object.height / 2.0) as f32;
                btn.button.game_object.width = 600.0;
                btn.button.game_object.x = app.width as f32 / 2.0 + 50.0;
            } else {
                btn.button.color = Color::RGB(100, 100, 100);
                btn.button.game_object.width = 600.0;
                btn.button.game_object.x = app.width as f32 / 2.0 + 100.0;
            }
        }

        let mut back_pos = 70;
        let mut start_back = 100;

        for back_btns in (0..self.actual_button).rev() {
            self.btn_list[back_btns].button.game_object.y = (app.height as f32 / 2.0) - (self.btn_list[back_btns].button.game_object.height / 2.0) - back_pos as f32;
            self.btn_list[back_btns].button.game_object.x = app.width as f32 / 2.0 + start_back as f32;
            back_pos += 60;
            start_back += 20;
        }

        let mut front_pos = 70;
        let mut start_back = 100;
        
        for back_btns in self.actual_button + 1..self.btn_list.len() {
            self.btn_list[back_btns].button.game_object.y = (app.height as f32 / 2.0) - (self.btn_list[back_btns].button.game_object.height / 2.0) + front_pos as f32;
            self.btn_list[back_btns].button.game_object.x = app.width as f32 / 2.0 + start_back as f32;
            front_pos += 60;
            start_back += 20;
        }

        Self::event_handler(self,app_state, event_pump,  _font, app);
    }

    fn event_handler(&mut self, app_state: &mut AppState, event_pump: &mut sdl2::EventPump, _font: &Font, app: &mut App) {
        let mut texture_creator = app.canvas.texture_creator();
        for event in event_pump.poll_iter() {
            match event { 
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[0]).unwrap() => {
                    app_state.state = GameState::MainMenu;
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[1]).unwrap() => {
                    if self.actual_button > usize::MIN {
                        self.actual_button -= 1;
                    } else {
                        self.actual_button = self.btn_list.len() - 1;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[2]).unwrap() => {
                    if self.actual_button < self.btn_list.len() - 1 {
                        self.actual_button += 1;
                    } else {
                        self.actual_button = 0;
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Delete), .. } => {
                    self.loading(&mut texture_creator, _font, &mut app.canvas);
                    match &self.btn_list[self.actual_button].button.text {
                        Some(_text) => {
                            app_state.song_folder = Some(_text.clone());
                        },
                        None => {},
                    }
                    app.reseted = false;
                    app_state.state = GameState::Editing;
            },
                sdl2::event::Event::KeyDown { keycode: Some(key_value), .. } if key_value == Keycode::from_i32(app.play_keys[3]).unwrap() => {
                        self.loading(&mut texture_creator, _font, &mut app.canvas);
                        match &self.btn_list[self.actual_button].button.text {
                            Some(_text) => {
                                app_state.song_folder = Some(_text.clone());
                            },
                            None => {},
                        }
                        app.reseted = false;
                        app_state.state = GameState::Playing;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. }  => {
                    self.loading(&mut texture_creator, _font, &mut app.canvas);
                        match &self.btn_list[self.actual_button].button.text {
                            Some(_text) => {
                                app_state.song_folder = Some(_text.clone());
                            },
                            None => {},
                        }
                        app.reseted = false;
                        app_state.state = GameState::SongCalibration;
                },
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    app_state.state = GameState::MainMenu;
                },
                _ => {}
            }
            /* 
            for btn in 0..self.btn_list.len() {
                if self.btn_list[btn].on_click(&event) || self.btn_list[btn].on_lclick(&event) {
                    // add here a reset for the play
                    self.loading(&mut texture_creator, _font, &mut app.canvas);
                    match &self.btn_list[btn].text {
                        Some(_text) => {
                            app_state.song_folder = Some(_text.clone());
                        },
                        None => {},
                    }

                    if self.btn_list[btn].on_lclick(&event) && app.can_edit {
                        app.reseted = false;
                        app_state.state = GameState::Editing;
                    } else {
                        app.reseted = false;
                        app_state.state = GameState::Playing;
                    }
                }
            }
            */
        }
    }

    // this will show the loading message
    fn loading (&mut self, texture_creator: &mut TextureCreator<WindowContext>, _font: &Font, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        self.loading_text.render(canvas, texture_creator, _font);
        canvas.present();
    }

    fn load_songs(songs_folder: String, app: &mut App) -> Vec<SongFile>{
        let folders = fs::read_dir(&songs_folder).unwrap();
        let mut songs_buttons: Vec<SongFile> = vec![];
        let mut position = 50.0;

        for entry in folders {
            let entry = entry.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    println!("{}", entry.file_name().to_string_lossy().to_string() + "/cover.png");
                    
                    let mut img_texture = None;
                    let path = "./songs/".to_owned() + &entry.file_name().to_string_lossy().to_string() + "/cover";
                    if Path::new(&(path.clone() + ".png")).exists() {
                        img_texture = app.texture_creator.load_texture(&(path + ".png")).ok();
                    } else if Path::new(&(path.clone() + ".jpeg")).exists() {
                        img_texture = app.texture_creator.load_texture(&(path + ".jpeg")).ok();
                    } else if Path::new(&(path.clone() + ".jpg")).exists() {
                        img_texture = app.texture_creator.load_texture(&(path + ".jpg")).ok();
                    } else if Path::new(&(path.clone() + ".svg")).exists() {
                        img_texture = app.texture_creator.load_texture(&(path + ".svg")).ok();
                    } else if Path::new(&(path.clone() + ".gif")).exists() {
                        img_texture = app.texture_creator.load_texture(&(path + ".gif")).ok();
                    }

                    songs_buttons.push(
                        SongFile {
                            button: Button::new(
                                GameObject {
                                    active: true,
                                    x: app.width as f32 - 350.0,
                                    y: position,
                                    width: 350.0,
                                    height: 50.0,
                                },
                                Some(entry.file_name().to_string_lossy().to_string()),
                                Color::RGB(100, 100, 100),
                                Color::WHITE,
                                Color::RGB(0, 200, 0),
                                Color::RGB(0, 0, 0),
                                None,
                                TextAlign::Center),
                                img_texture
                        }
                        
                    );
                    position += 60.0;
                }
            }
        return songs_buttons
    }
}

