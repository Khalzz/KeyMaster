
use std::sync::{Arc, Mutex};
use std::{env, thread};
use std::fs::File;
use std::io::{Error, Read};
use std::thread::current;
use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use rodio::{Decoder, Source};
use rustfft::algorithm::Radix4;
use rustfft::num_complex::{self, Complex, Complex32};
use rustfft::num_traits::Zero;
use rustfft::{Fft, FftDirection, FftPlanner, Length};
use sdl2::audio::{AudioCallback, AudioQueue, AudioSpecDesired};
use sdl2::image::LoadTexture;
use sdl2::messagebox::MessageBoxFlag;
use sdl2::mixer::{DEFAULT_CHANNELS, AUDIO_S16LSB, DEFAULT_FORMAT, InitFlag, self, DEFAULT_FREQUENCY, Sdl2MixerContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2::AudioSubsystem;
use sdl2::{video::Window, Sdl, render::Canvas, sys::KeyCode, keyboard::Keycode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::game_object::GameObject;
use crate::gameplay::play;
use crate::gameplay::editor;
use crate::gameplay::main_menu;
use crate::gameplay::settings;
use crate::gameplay::song_selector;
use crate::gameplay::controller;
use crate::gameplay::calibration;
use crate::gameplay::manual_calibration;
use crate::input::button_module::{Button, TextAlign};
use crate::load_song::Song;

const NUM_BARS: usize = 20;
// const LOGO: &[u8] = include_bytes!("assets/non_modifiable_image.png")

// in this file we will have the main work flow of our app, as a struct defined mainly to do what we want to do:
pub enum GameState {
    MainMenu,
    Settings,
    Playing,
    Editing,
    Calibrating,
    ManualCalibrating,
    Controlers,
    Quitting,
    SelectingSong,
}

pub struct CoordinationData {
    pub base_time: i128,
    pub end_time: i128,
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

pub struct Textures {
    pub red_key: Option<Texture>,
    pub yellow_key: Option<Texture>,
    pub purple_key: Option<Texture>,
    pub blue_key: Option<Texture>,
    pub red_note: Option<Texture>,
    pub yellow_note: Option<Texture>,
    pub purple_note: Option<Texture>,
    pub blue_note: Option<Texture>,
    pub mute_note: Option<Texture>,
    pub red_hold: Option<Texture>,
    pub yellow_hold: Option<Texture>,
    pub purple_hold: Option<Texture>,
    pub blue_hold: Option<Texture>,
    pub mute_hold: Option<Texture>,
    pub background: Option<Texture>,
}

pub struct Visualizer {
    pub bars: bool,
    pub circle: bool
}

pub struct App {
    pub context: Sdl,
    pub mixer_context: (),
    pub audio_subsystem: AudioSubsystem,
    pub width: u32,
    pub height: u32,
    pub canvas: Canvas<Window>,
    pub coordination_data: CoordinationData,
    pub play_keys: [i32; 4],
    pub volume_percentage: i32,
    pub paused: bool,
    pub start_pause: Instant,
    pub paused_time: u128,
    pub reseted: bool,
    pub testing_song: Option<Testing>,
    pub calibrate_on_start: bool,
    pub alert_message: String,
    pub can_edit: bool,
    pub texture_creator: TextureCreator<WindowContext>,
    pub textures: Textures,
    pub visualizer_settings: Visualizer,
    pub ctrl_string: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameController {
    pub(crate) controller_array: [i32; 4],
}

impl App {
    pub fn new(title: &str) -> App{
        // base sdl2
        let context = sdl2::init().expect("SDL2 wasn't initialized");
        let video_susbsystem = context.video().expect("The Video subsystem wasn't initialized");
        let audio_subsystem = context.audio().expect("The audio subsystem didnt loaded right");

        // sdl2_mixer
        mixer::init(InitFlag::FLAC | InitFlag::MOD | InitFlag::MP3 | InitFlag::OGG).expect("Failed to initialize SDL2_mixer");
        let mixer_context = mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).expect("Failed to open audio device");
        mixer::allocate_channels(DEFAULT_CHANNELS);
        let current_display = video_susbsystem.current_display_mode(0).unwrap();
        
        let width = current_display.w as u32;
        let height = current_display.h as u32;

        env::set_var("SDL_VIDEO_MINIMIZE_ON_FOCUS_LOSS", "0"); // this is highly needed so the sdl2 can alt tab without generating bugs

        let window = video_susbsystem.window(title, width, height as u32).vulkan().fullscreen().build().expect("The window wasn't created");
        let mut canvas = window.into_canvas().accelerated().present_vsync().build().expect("the canvas wasn't builded");
        
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let texture_creator = canvas.texture_creator();

        let textures = Textures { 
            red_key: texture_creator.load_texture("assets/sprites/RedKey-Sheet.png").ok(),
            yellow_key: texture_creator.load_texture("assets/sprites/YellowKey-Sheet.png").ok(),
            purple_key: texture_creator.load_texture("assets/sprites/PurpleKey-Sheet.png").ok(), 
            blue_key: texture_creator.load_texture("assets/sprites/BlueKey-Sheet.png").ok(), 
            red_note: texture_creator.load_texture("assets/sprites/notes/RedNote.png").ok(), 
            yellow_note: texture_creator.load_texture("assets/sprites/notes/YellowNote.png").ok(), 
            purple_note: texture_creator.load_texture("assets/sprites/notes/PurpleNote.png").ok(), 
            blue_note: texture_creator.load_texture("assets/sprites/notes/BlueNote.png").ok(), 
            mute_note: texture_creator.load_texture("assets/sprites/notes/MutedNote.png").ok(),
            red_hold: texture_creator.load_texture("assets/sprites/hold/RedHold.png").ok(), 
            yellow_hold: texture_creator.load_texture("assets/sprites/hold/YellowHold.png").ok(), 
            purple_hold: texture_creator.load_texture("assets/sprites/hold/PurpleHold.png").ok(), 
            blue_hold: texture_creator.load_texture("assets/sprites/hold/BlueHold.png").ok(),
            mute_hold: texture_creator.load_texture("assets/sprites/hold/MuteHold.png").ok(),
            background: texture_creator.load_texture("assets/sprites/background.png").ok()
        };

        let mut settings = GameController { controller_array: [100,102,106,107] };

        match Self::load_settings() {
            Ok(settings_data) => settings = settings_data,
            Err(_) => {},
        }

        App {
            context,
            mixer_context,
            audio_subsystem,
            width,
            height,
            canvas,
            coordination_data: CoordinationData { base_time: 0, end_time: 0, key_speed: 700.0, saved_key_speed: 700.0},
            play_keys: settings.controller_array,
            volume_percentage: 15,
            paused: false,
            start_pause: Instant::now(),
            paused_time: 0,
            reseted: false,
            testing_song: None,
            calibrate_on_start: true,
            alert_message: String::from(""),
            can_edit: true,
            texture_creator,
            textures,
            visualizer_settings: Visualizer { bars: true, circle: true },
            ctrl_string: "".to_owned()
        }
    }

    

    pub fn render(mut self) {
        let mut app_state = AppState { is_running: true, state: GameState::Calibrating, song_folder: None };

        let fft_data = Arc::new(Mutex::new(vec![0.0; NUM_BARS]));
        let data_inside_closure = fft_data.clone();
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");

        let mut supported_configs_range = device.supported_output_configs().expect("error while querying configs");
        let supported_config = supported_configs_range.next().expect("no supported config?!").with_max_sample_rate();

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let sample_format = supported_config.sample_format();
        let config = supported_config.into();

        let mut controls = Button::new(
            GameObject {active: true, x:((self.width/2) - (100/2)) as f32, y: self.height as f32 - 60.0, width: 100.0, height: 50.0},
            Some(String::from(&self.ctrl_string)),
            Color::RGBA(100, 100, 100, 0),
            Color::WHITE,
            Color::RGB(0, 200, 0),
            Color::RGB(0, 0, 0),
            None, 
            TextAlign::Center
        );

        let mut last_update = Instant::now(); // Track the time of the last update

        let stream = match sample_format {
            SampleFormat::F32 => device.build_input_stream(&config, move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if last_update.elapsed().as_millis() > 10 && (self.visualizer_settings.circle ||  self.visualizer_settings.bars) {
                let mut locked_value = data_inside_closure.lock().unwrap();
                let mut planner: FftPlanner<f32> = FftPlanner::new();
                let fft = planner.plan_fft_forward(data.len());
                let mut buffer = vec![Complex::zero(); data.len()];
                fft.process(&mut buffer);

                let num_samples = data.len();
                // println!("{}", num_samples);
                let samples_per_bar = num_samples / NUM_BARS;
                const SMOOTHING_FACTOR: f32 = 0.5;
                
                for i in 0..NUM_BARS {
                    let start_index = i * samples_per_bar;
                    let end_index = (i + 1) * samples_per_bar;
                
                    let mut magnitude_sum = 0.0;
                    for &sample in data[start_index..end_index].iter() {
                        let magnitude = (&sample.powi(2) + &sample.powi(2)).sqrt() as f32;
                        magnitude_sum += magnitude;
                    }
                
                    let average_magnitude = magnitude_sum / samples_per_bar as f32;
                
                    if i > 0 {
                        locked_value[i] = locked_value[i - 1] * (1.0 - SMOOTHING_FACTOR) + average_magnitude * SMOOTHING_FACTOR;
                    } else {
                        locked_value[i] = average_magnitude;
                    }
                }
                last_update = Instant::now();
            }

        }, err_fn, None),
            sample_format => panic!("Unsupported sample format '{sample_format}'")
        }.unwrap();
        
        stream.play();

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
        let mut manual_calibration = manual_calibration::GameLogic::new(&mut self);

        mixer::Music::set_volume(((self.volume_percentage as f32 / 100.0) * 128.0) as i32);

        while app_state.is_running {
            controls.text = Some(self.controller_str());
            self.canvas.set_draw_color(Color::RGBA(40, 40, 40, 100));
            self.canvas.clear();
            
            match app_state.state {
                GameState::MainMenu => {
                    controls.render(&mut self.canvas, &self.texture_creator, &_font);
                    self.reseted = false; // every other option should need to "reset the reset value"
                    menu.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::SelectingSong => {
                    controls.render(&mut self.canvas, &self.texture_creator, &_font);
                    song_selector.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Playing => {
                    if !self.reseted {
                        play = play::GameLogic::new(&mut self, &mut app_state);
                        self.paused_time = 0;
                        play.start_time = Instant::now();
                        self.reseted = true;
                    }
                    play.update(&_font, &mut app_state, &mut event_pump, &mut self, fft_data.lock().unwrap());
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
                GameState::ManualCalibrating => {
                    if !self.reseted {
                        manual_calibration = manual_calibration::GameLogic::new(&mut self);
                        self.reseted = true;
                    }
                    manual_calibration.update(&_font, &mut app_state, &mut event_pump, &mut self);
                },
                GameState::Quitting => {
                    app_state.is_running = false;
                    mixer::close_audio();
                }
            }
            self.canvas.present();
        }
    }

    fn load_settings() -> Result<GameController, Error> {
        let mut settings = GameController { controller_array: [100,102,106,107] };
        match std::fs::read_to_string("settings.json") {
            Ok(file_contents) => {
                settings = serde_json::from_str(&file_contents)?;
            },
            Err(_) => {
                eprintln!("The settings Json didn't loaded correctly");
            },
        }
        Ok(settings)
    }

    fn controller_str(&mut self) -> String {
        let mut ctrl_string = "".to_owned();

        for (i, key) in self.play_keys.iter().enumerate() {
            match Keycode::from_i32(*key) {
                Some(keycode) => {
                    let mut ctrl_opt = "";

                    if i == 0 {
                        ctrl_opt = "-[back], ";
                    } else if i == 1 {
                        ctrl_opt = "-[up], ";
                    } else if i == 2 {
                        ctrl_opt = "-[down], ";
                    } else if i == 3 {
                        ctrl_opt = "-[select]";
                    }

                    ctrl_string += &(keycode.to_string() + ctrl_opt);
                },
                None => {},
            }
        }

        return ctrl_string;
    }
}