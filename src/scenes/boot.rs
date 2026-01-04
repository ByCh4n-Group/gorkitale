#![allow(clippy::collapsible_if)]
use crate::assets::{ASSET_LIST, AssetType};
use crate::defs::{Scene, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::game_state::GameState;
use crate::global_db::GlobalSettings;
use tetra::Context;
use tetra::audio::{Sound, SoundInstance};
use tetra::graphics::text::Text;
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;

const INTRO_FPS: f64 = 30.0;

pub struct BootState {
    pub asset_index: usize,
    pub loading_complete: bool,
    pub audio_muted: bool,
    pub waiting_for_input: bool,
    
    // Animation frames
    frames: Vec<Texture>,
    current_frame: usize,
    frame_timer: f64,
    animation_ended: bool,
    
    intro_sound: Option<Sound>,
    intro_instance: Option<SoundInstance>,
    initialized: bool,
    
    // Pulsing timer for "Press Enter" text
    pulse_timer: f32,
    
    // Delay counter to allow first draw
    startup_frames: u32,
}

impl BootState {
    pub fn new() -> Self {
        Self {
            asset_index: 0,
            loading_complete: false,
            audio_muted: false,
            waiting_for_input: false,
            frames: Vec::new(),
            current_frame: 0,
            frame_timer: 0.0,
            animation_ended: false,
            intro_sound: None,
            intro_instance: None,
            initialized: false,
            pulse_timer: 0.0,
            startup_frames: 0,
        }
    }

    fn init_intro(&mut self, ctx: &mut Context) {
        if self.initialized {
            return;
        }
        self.initialized = true;
        
        // Skip intro in debug/dev mode
        if cfg!(debug_assertions) {
            println!("Debug mode detected: Skipping intro animation.");
            return;
        }

        let settings = GlobalSettings::load();
        let (gif_path, audio_path) = if settings.language == "tr" {
            ("assets/intro_tr.gif", "assets/intro_tr.mp3")
        } else {
            ("assets/intro_en.gif", "assets/intro_en.mp3")
        };

        // Load animation frames from GIF
        self.load_gif(ctx, gif_path);

        // Load intro audio
        match Sound::new(audio_path) {
            Ok(sound) => {
                self.intro_sound = Some(sound);
                println!("Loaded intro audio: {}", audio_path);
            }
            Err(e) => {
                println!("Could not load intro audio '{}': {}", audio_path, e);
            }
        }

        // Play the audio
        if !self.audio_muted {
            if let Some(sound) = &self.intro_sound {
                match sound.play_with(ctx, 1.0, 1.0) {
                    Ok(instance) => {
                        println!("Intro audio started.");
                        self.intro_instance = Some(instance);
                    }
                    Err(e) => println!("Failed to play intro audio: {}", e),
                }
            }
        }
    }

    fn load_gif(&mut self, ctx: &mut Context, path: &str) {
        use std::fs::File;
        use gif::{DecodeOptions, DisposalMethod};
        use tetra::graphics::TextureFormat;

        println!("Loading GIF: {}", path);
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to open GIF '{}': {}", path, e);
                return;
            }
        };

        let mut options = DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        
        let mut decoder = match options.read_info(file) {
            Ok(d) => d,
            Err(e) => {
                println!("Failed to read GIF info: {}", e);
                return;
            }
        };

        let width = decoder.width() as usize;
        let height = decoder.height() as usize;
        let mut canvas = vec![0u8; width * height * 4]; // RGBA buffer

        while let Some(frame) = decoder.read_next_frame().unwrap_or(None) {
            let frame_left = frame.left as usize;
            let frame_top = frame.top as usize;
            let frame_width = frame.width as usize;
            let frame_height = frame.height as usize;
            
            // Backup canvas for RestorePrevious disposal
            let previous_canvas = if let DisposalMethod::Previous = frame.dispose {
                Some(canvas.clone())
            } else {
                None
            };

            // Draw frame onto canvas
            for y in 0..frame_height {
                for x in 0..frame_width {
                    if frame_top + y >= height || frame_left + x >= width { continue; }
                    
                    let canvas_idx = ((frame_top + y) * width + (frame_left + x)) * 4;
                    let frame_idx = (y * frame_width + x) * 4;
                    
                    if frame_idx + 4 <= frame.buffer.len() {
                        let pixel = &frame.buffer[frame_idx..frame_idx+4];
                        // Simple alpha blending (if alpha > 0, overwrite)
                        if pixel[3] > 0 {
                            canvas[canvas_idx..canvas_idx+4].copy_from_slice(pixel);
                        }
                    }
                }
            }

            match Texture::from_data(ctx, width as i32, height as i32, TextureFormat::Rgba8, &canvas) {
                 Ok(tex) => self.frames.push(tex),
                 Err(e) => println!("Failed to create texture from GIF frame: {}", e),
             }
             
             // Handle disposal
             match frame.dispose {
                 DisposalMethod::Background => {
                     for y in 0..frame_height {
                        for x in 0..frame_width {
                            if frame_top + y >= height || frame_left + x >= width { continue; }
                            let canvas_idx = ((frame_top + y) * width + (frame_left + x)) * 4;
                            canvas[canvas_idx..canvas_idx+4].fill(0);
                        }
                    }
                 },
                 DisposalMethod::Previous => {
                     if let Some(prev) = previous_canvas {
                         canvas = prev;
                     }
                 },
                 _ => {}
             }
        }
        
        println!("Loaded {} GIF frames", self.frames.len());
    }
}

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Wait a few frames before initializing intro to allow first draw
    if state.boot_state.startup_frames < 2 {
        state.boot_state.startup_frames += 1;
        return Ok(());
    }

    // Initialize intro
    if !state.boot_state.initialized {
        state.boot_state.init_intro(ctx);
    }

    // Handle Mute toggle
    if input::is_key_pressed(ctx, Key::S) || input::is_key_pressed(ctx, Key::M) {
        state.boot_state.audio_muted = !state.boot_state.audio_muted;
        if let Some(instance) = &mut state.boot_state.intro_instance {
            if state.boot_state.audio_muted {
                instance.set_volume(0.0);
            } else {
                instance.set_volume(1.0);
            }
        }
    }

    // Update animation
    if !state.boot_state.frames.is_empty() && !state.boot_state.animation_ended {
        let dt = tetra::time::get_delta_time(ctx).as_secs_f64();
        state.boot_state.frame_timer += dt;
        
        let frame_duration = 1.0 / INTRO_FPS;
        
        while state.boot_state.frame_timer >= frame_duration {
            state.boot_state.frame_timer -= frame_duration;
            state.boot_state.current_frame += 1;
            
            if state.boot_state.current_frame >= state.boot_state.frames.len() {
                // Loop animation
                state.boot_state.current_frame = 0;
                
                // Restart audio to sync with loop
                if let Some(instance) = &mut state.boot_state.intro_instance {
                    instance.stop();
                }
                
                if let Some(sound) = &state.boot_state.intro_sound {
                    if !state.boot_state.audio_muted {
                        match sound.play_with(ctx, 1.0, 1.0) {
                            Ok(instance) => {
                                state.boot_state.intro_instance = Some(instance);
                            }
                            Err(e) => println!("Failed to restart intro audio: {}", e),
                        }
                    }
                }
            }
        }
    }

    // Update pulse timer unconditionally
    state.boot_state.pulse_timer += tetra::time::get_delta_time(ctx).as_secs_f32();

    // Asset Loading (one per frame for smooth animation)
    if !state.boot_state.loading_complete {
        if state.boot_state.asset_index < ASSET_LIST.len() {
            let def = &ASSET_LIST[state.boot_state.asset_index];
            
            match def.asset_type {
                AssetType::Texture => {
                    if let Ok(tex) = Texture::new(ctx, def.path) {
                        state.texture_cache.insert(def.name.to_string(), tex.clone());
                        state.assign_texture(def.name, tex);
                    } else {
                        println!("Failed to load texture: {}", def.path);
                    }
                }
                AssetType::Sound => {
                    if let Ok(snd) = Sound::new(def.path) {
                        state.sound_cache.insert(def.name.to_string(), snd.clone());
                        state.assign_sound(def.name, snd);
                    } else {
                        println!("Failed to load sound: {}", def.path);
                    }
                }
            }
            
            state.boot_state.asset_index += 1;
        } else {
            state.boot_state.loading_complete = true;
        }
    }

    // Transition Logic
    if state.boot_state.loading_complete {
        state.boot_state.waiting_for_input = true;
        
        if input::is_key_pressed(ctx, Key::Enter) || input::is_key_pressed(ctx, Key::Space) {
            // Stop intro audio
            if let Some(instance) = &mut state.boot_state.intro_instance {
                instance.stop();
            }
            state.scene = Scene::Menu;
        }
    }

    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    graphics::clear(ctx, Color::BLACK);

    // Draw current animation frame
    if !state.boot_state.frames.is_empty() {
        let frame_idx = state.boot_state.current_frame.min(state.boot_state.frames.len() - 1);
        let tex = &state.boot_state.frames[frame_idx];
        
        let tex_width = tex.width() as f32;
        let tex_height = tex.height() as f32;
        
        let scale_x = SCREEN_WIDTH as f32 / tex_width;
        let scale_y = SCREEN_HEIGHT as f32 / tex_height;
        
        tex.draw(ctx, tetra::graphics::DrawParams::new()
            .scale(Vec2::new(scale_x, scale_y)));
    } else {
        // Fallback: show loading text if no frames
        let text = "GORKITALE";
        let mut t = Text::new(text, state.font.clone());
        let bounds = t.get_bounds(ctx).unwrap();
        let scale = 3.5;
        let pos = Vec2::new(
            (SCREEN_WIDTH as f32 - bounds.width * scale) / 2.0,
            (SCREEN_HEIGHT as f32 - bounds.height * scale) / 2.0,
        );
        
        let alpha = if cfg!(debug_assertions) {
            // Soft breathing effect for dev profile
            let time = state.boot_state.pulse_timer;
            (time * 2.0).sin() * 0.3 + 0.7
        } else {
            1.0
        };

        t.draw(ctx, tetra::graphics::DrawParams::new()
            .position(pos)
            .scale(Vec2::new(scale, scale))
            .color(Color::rgba(1.0, 1.0, 1.0, alpha)));
    }

    // Draw Loading Progress (bottom right corner)
    if !state.boot_state.loading_complete {
        let current = state.boot_state.asset_index;
        let total = ASSET_LIST.len();
        let filename = if current < total {
            ASSET_LIST[current].path
        } else {
            "Done"
        };
        
        let progress_text = format!("{} [{}/{}]", filename, current, total);
        let mut text = Text::new(progress_text, state.font.clone());
        let bounds = text.get_bounds(ctx).unwrap();
        let pos = Vec2::new(
            SCREEN_WIDTH as f32 - bounds.width - 10.0,
            SCREEN_HEIGHT as f32 - bounds.height - 10.0,
        );
        // Draw with black outline for visibility
        text.draw(ctx, tetra::graphics::DrawParams::new()
            .position(pos + Vec2::new(1.0, 1.0))
            .color(Color::BLACK));
        text.draw(ctx, pos);
    }

    // Draw "Press Enter" prompt
    if state.boot_state.waiting_for_input {
        let msg = "Press Enter to continue";
        let mut text = Text::new(msg, state.font.clone());
        let bounds = text.get_bounds(ctx).unwrap();
        let pos = Vec2::new(
            SCREEN_WIDTH as f32 - bounds.width - 20.0,
            SCREEN_HEIGHT as f32 - bounds.height - 20.0,
        );
        // Pulsing effect
        let alpha = (state.boot_state.pulse_timer * 3.0).sin() * 0.3 + 0.7;
        
        text.draw(ctx, tetra::graphics::DrawParams::new()
            .position(pos + Vec2::new(1.0, 1.0))
            .color(Color::rgba(0.0, 0.0, 0.0, alpha)));
        text.draw(ctx, tetra::graphics::DrawParams::new()
            .position(pos)
            .color(Color::rgba(1.0, 1.0, 1.0, alpha)));
    }

    // Draw mute indicator
    if state.boot_state.audio_muted {
        let mut text = Text::new("[MUTED - Press M to unmute]", state.font.clone());
        text.draw(ctx, Vec2::new(10.0, 10.0));
    }

    Ok(())
}
