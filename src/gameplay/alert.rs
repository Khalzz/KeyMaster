use std::time::{Duration, Instant};
use sdl2::{event::{self, Event}, image::LoadTexture, keyboard::Keycode, mixer::{self, Music}, pixels::Color, render::{Canvas, TextureCreator}, sys::ttf::TTF_FontHeight, ttf::Font, video::{Window, WindowContext}};
use crate::{app::{App, AppState, GameState, self}, game_object::GameObject, input::button_module::Button, input::keybutton::{KeyButton}, key::GameKey, load_song::Song, UI::{text::Label}};

pub struct GameLogic { // here we define the data we use on our script
    error_label: Label,
    ok_button: Button,
} 

impl GameLogic {
    // this is called once
    pub fn new(app: &mut App) -> Self {
        let error_label = Label::new(GameObject { active: true, x: 0.0, y: 0.0, width: app.width as f32, height: app.height as f32 }, app.alert_message.clone(),Color::RGBA(0, 0, 0, 0),Color::RGB(255, 255, 255));
        let ok_button = Button::new(
            GameObject {
                active: true, x:((app.width/2) - (100/2)) as f32, y: (app.height as f32/2.0) + 200.0 as f32, width: 100.0, height: 50.0},
            Some(String::from("OK")),
            Color::RGB(100, 100, 100),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None
        );
        Self {
            error_label,
            ok_button
        }
    }

    pub fn update(&mut self, _font: &Font, mut app_state: &mut AppState, mut event_pump: &mut sdl2::EventPump, app: &mut App) {
        self.error_label.text = app.alert_message.clone();
        self.error_label.render(&mut app.canvas, &app.texture_creator, _font);
        self.ok_button.render(&mut app.canvas, &app.texture_creator, _font);

        Self::event_handler(self, &mut app_state, &mut event_pump, app);
    }

    fn event_handler(&mut self,app_state: &mut AppState, event_pump: &mut sdl2::EventPump, app: &mut App) {

        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => {
                    app_state.is_running = false;
                } 
                _ => {}
            }

            self.ok_button.is_hover(&event);

            if self.ok_button.on_click(&event) {
                Self::reset(app, app_state)
            }
        }
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
}