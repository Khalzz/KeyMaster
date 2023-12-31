
use std::time::Instant;

use sdl2::mixer::{DEFAULT_CHANNELS, AUDIO_S16LSB, DEFAULT_FORMAT, InitFlag, self, DEFAULT_FREQUENCY, Sdl2MixerContext};
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{video::Window, Sdl, render::Canvas, sys::KeyCode, keyboard::Keycode};

use crate::gameplay::play;
use crate::gameplay::editor;
use crate::gameplay::main_menu;
use crate::gameplay::settings;
use crate::gameplay::song_selector;
use crate::gameplay::controller;
use crate::gameplay::calibration;
use crate::load_song::Song;

// in this file we will have the main work flow of our app, as a struct defined mainly to do what we want to do:
pub enum GameState {
    MainMenu,
    Settings,
    Playing,
    Editing,
    Calibrating,
    Controlers,
    Quitting,
    SelectingSong
}

pub struct CoordinationData {
    pub base_time: u128,
    pub end_time: u128,
    pub key_speed: f32,
    pub saved_key_speed: f32,
}

pub struct AppState {
    pub is_running: bool,
    pub state: GameState,
    pub song_folder: Option<String>
}

pub struct Testing {
    pub song: Song,
    pub start_point: f64
}

pub struct App {
    pub context: Sdl,
    pub mixer_context: (),  
    pub width: u32,
    pub height: u32,
    pub canvas: Canvas<Window>,
    pub coordination_data: CoordinationData,
    pub play_keys: [Keycode; 4],
    pub texture_creator: TextureCreator<WindowContext>,
    pub volume_percentage: i32,
    pub paused: bool,
    pub start_pause: Instant,
    pub paused_time: u128,
    pub reseted: bool,
    pub testing_song: Option<Testing>
}

impl App {
    pub fn new(width: u32, height: u32,  title: &str) -> App{
        // base sdl2
        let context = sdl2::init().expect("SDL2 wasn't initialized");
        let video_susbsystem = context.video().expect("The Video subsystem wasn't initialized");

        // sdl2_mixer
        mixer::init(InitFlag::FLAC | InitFlag::MOD | InitFlag::MP3 | InitFlag::OGG).expect("Failed to initialize SDL2_mixer");
        let mixer_context = mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).expect("Failed to open audio device");
        mixer::allocate_channels(DEFAULT_CHANNELS);

        let window = video_susbsystem.window(title, width, height).opengl().fullscreen_desktop().build().expect("The window wasn't created");
        let mut canvas = window.into_canvas().build().expect("the canvas wasn't builded");

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let texture_creator = canvas.texture_creator();
        
        return App {
            context,
            mixer_context,
            width,
            height,
            canvas: canvas,
            coordination_data: CoordinationData { base_time: 0, end_time: 0, key_speed: 700.0, saved_key_speed: 700.0},
            play_keys: [Keycode::D, Keycode::F, Keycode::J, Keycode::K],
            texture_creator,
            volume_percentage: 15,
            paused: false,
            start_pause: Instant::now(),
            paused_time: 0,
            reseted: false,
            testing_song: None
        }
    }

    pub fn render(mut self) {
        let mut app_state = AppState { is_running: true, state: GameState::Calibrating, song_folder: None };

        // here we will make the rendering of everything
        let mut event_pump = self.context.event_pump().unwrap();

        // for the TEXT handling
        let ttf_context = sdl2::ttf::init().unwrap(); // we create a "context"
        let use_font = "./assets/fonts/Inter-Thin.ttf";
        let mut _font = ttf_context.load_font(use_font, 20).unwrap();

        let mut menu = main_menu::GameLogic::new(&mut self);
        let mut settings = settings::GameLogic::new(&mut self);
        let mut controller = controller::GameLogic::new(&mut self);
        let mut song_selector = song_selector::GameLogic::new(&mut self);
        let mut play = play::GameLogic::new(&mut self, &mut app_state);
        let mut editor = editor::GameLogic::new(&mut self, &mut app_state, &_font);
        let mut calibration = calibration::GameLogic::new(&mut self);

        mixer::Music::set_volume(((self.volume_percentage as f32 / 100.0) * 128.0) as i32);

        while app_state.is_running {
            match app_state.state {
                GameState::MainMenu => {
                    self.reseted = false; // every other option should need to "reset the reset value"
                    menu.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::SelectingSong => {
                    song_selector.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Playing => {
                    if !self.reseted {
                        play = play::GameLogic::new(&mut self, &mut app_state);
                        self.paused_time = 0;
                        play.start_time = Instant::now();
                        self.reseted = true;
                    }
                    play.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Editing => {
                    if !self.reseted {
                        editor = editor::GameLogic::new(&mut self, &mut app_state, &_font);
                        self.reseted = true;
                    }
                    editor.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Settings => {
                    self.reseted = false; // every other option should need to "reset the reset value"
                    settings.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Controlers => {
                    controller.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Calibrating => {
                    if !self.reseted {
                        calibration = calibration::GameLogic::new(&mut self);
                        self.reseted = true;
                    }
                    calibration.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Quitting => {
                    app_state.is_running = false;
                    mixer::close_audio();
                }
                
            }
        }
    }

    
}