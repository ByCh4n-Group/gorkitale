use crate::assets::{ASSET_LIST, AssetType};
use crate::defs::{Scene, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::game_state::GameState;
use crate::global_db::GlobalSettings;
use tetra::Context;
use tetra::audio::{Sound, SoundInstance};
use tetra::graphics::text::Text;
use tetra::graphics::{self, Color, Texture, TextureFormat};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use video_rs::{Decoder, Location};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, sync_channel, channel};
use std::thread;

type VideoFrame = (f64, Vec<u8>, u32, u32, f64);

pub struct BootState {
    pub asset_index: usize,
    pub loading_complete: bool,
    pub video_muted: bool,
    pub waiting_for_input: bool,
    
    video_receiver: Option<Receiver<VideoFrame>>,
    recycler_tx: Option<Sender<Vec<u8>>>,
    next_frame: Option<VideoFrame>,
    video_sound: Option<Sound>,
    video_instance: Option<SoundInstance>,
    
    video_texture: Option<Texture>,
    video_width: u32,
    video_height: u32,
    video_fps: f64,
    video_timer: f64,
    video_ended: bool,
}

impl BootState {
    pub fn new() -> Self {
        Self {
            asset_index: 0,
            loading_complete: false,
            video_muted: false,
            waiting_for_input: false,
            video_receiver: None,
            recycler_tx: None,
            next_frame: None,
            video_sound: None,
            video_instance: None,
            video_texture: None,
            video_width: 0,
            video_height: 0,
            video_fps: 30.0,
            video_timer: 0.0,
            video_ended: false,
        }
    }

    fn init_video(&mut self, ctx: &mut Context) {
        let settings = GlobalSettings::load();
        let (filename, audio_filename) = if settings.language == "tr" {
            ("assets/intro_tr.mp4", "assets/intro_tr.mp3")
        } else {
            ("assets/intro_en.mp4", "assets/intro_en.mp3")
        };

        // Try to load audio from the separate MP3 file
        // We prioritize the MP3 file because the MP4 video likely has no audio track (we stripped it)
        // or has an unsupported codec (AAC).
        if self.video_sound.is_none() {
            if let Ok(sound) = Sound::new(audio_filename) {
                self.video_sound = Some(sound);
                println!("Loaded audio from: {}", audio_filename);
            } else {
                println!("AUDIO WARNING: Could not load audio from '{}'.", audio_filename);
                // Fallback: Try loading from video file just in case
                if let Ok(sound) = Sound::new(filename) {
                    self.video_sound = Some(sound);
                    println!("Loaded audio from video file: {}", filename);
                }
            }
        }

        // Play the sound
        #[allow(clippy::collapsible_if)]
        if let Some(sound) = &self.video_sound {
            if !self.video_muted {
                println!("Attempting to play audio loop...");
                match sound.play(ctx) {
                    Ok(instance) => {
                        println!("Audio loop started.");
                        self.video_instance = Some(instance);
                    }
                    Err(e) => println!("Failed to play audio loop: {}", e),
                }
            } else {
                println!("Audio is muted, skipping playback.");
            }
        } else {
            println!("No audio loaded to play!");
        }

        // Initialize video decoder in a separate thread
        let (tx, rx) = sync_channel(5);
        let (recycler_tx, recycler_rx) = channel();
        
        self.video_receiver = Some(rx);
        self.recycler_tx = Some(recycler_tx);
        
        let filename_owned = filename.to_string();
        
        thread::spawn(move || {
            let _ = video_rs::init(); 
            let source = Location::File(PathBuf::from(filename_owned));
            
            match Decoder::new(&source) {
                 Ok(mut decoder) => {
                     let (width, height) = decoder.size();
                     let fps = decoder.frame_rate() as f64;
                     let fps = if fps <= 0.0 { 30.0 } else { fps };
                     
                     for frame_result in decoder.decode_iter() {
                         if let Ok((time, frame)) = frame_result {
                             let rgb: &[u8] = frame.as_slice().unwrap();
                             
                             // Try to reuse a buffer from the recycler
                             let mut rgba = recycler_rx.try_recv().unwrap_or_else(|_| vec![0u8; (width * height * 4) as usize]);
                             
                             // Ensure buffer size is correct (in case resolution changed or new buffer)
                             if rgba.len() != (width * height * 4) as usize {
                                 rgba.resize((width * height * 4) as usize, 0);
                             }
                             
                             // Convert RGB to RGBA (Off-thread!)
                             // Optimized loop: using iterators to avoid bounds checks
                             for (i, chunk) in rgb.chunks_exact(3).enumerate() {
                                 let j = i * 4;
                                 // Safety: We resized rgba above to be exactly 4/3 of rgb size.
                                 // But to be safe and avoid unsafe blocks, we use direct indexing which is fast enough in release.
                                 if let Some(dest) = rgba.get_mut(j..j+4) {
                                     dest[0] = chunk[0];
                                     dest[1] = chunk[1];
                                     dest[2] = chunk[2];
                                     dest[3] = 255;
                                 }
                             }
                             
                             // Send to main thread (blocks if buffer full)
                             if tx.send((time.as_secs_f64(), rgba, width, height, fps)).is_err() {
                                 break;
                             }
                         } else {
                             break;
                         }
                     }
                 },
                 Err(e) => println!("Failed to create decoder: {}", e),
            }
        });
    }
}

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Init video on first frame
    if state.boot_state.video_receiver.is_none() && !state.boot_state.video_ended {
        state.boot_state.init_video(ctx);
    }

    // Handle Mute
    if input::is_key_pressed(ctx, Key::S) || input::is_key_pressed(ctx, Key::Space) {
        state.boot_state.video_muted = !state.boot_state.video_muted;
        // Toggle sound if playing
        // Note: Tetra Sound instances are fire-and-forget usually, but if we kept the instance we could control it.
        // For now, simple mute toggle for future plays or if we had a handle.
    }

    // Process Video Frames
    if let Some(rx) = &state.boot_state.video_receiver {
        let dt = tetra::time::get_delta_time(ctx).as_secs_f64();
        state.boot_state.video_timer += dt;
        
        // Fetch next frame if we don't have one
        if state.boot_state.next_frame.is_none() {
            match rx.try_recv() {
                Ok(frame_data) => state.boot_state.next_frame = Some(frame_data),
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    state.boot_state.video_ended = true;
                }
                _ => {} // Empty
            }
        }

        // Check if it's time to display the next frame
        if let Some((_timestamp, _, _, _, fps)) = state.boot_state.next_frame {
            // Sync logic: If video_timer is close to or past the frame timestamp
            // Note: video-rs timestamps are absolute.
            
            // First frame setup
            if state.boot_state.video_fps != fps {
                state.boot_state.video_fps = fps;
            }

            // Simple timer based sync (ignoring absolute timestamp for simplicity to match audio start)
            // Or use the timestamp from decoder? Decoder timestamp is better.
            // Let's use relative timer.
            
            let frame_duration = 1.0 / state.boot_state.video_fps;
            
            if state.boot_state.video_timer >= frame_duration {
                // Consume the frame
                if let Some((_, rgba, width, height, _)) = state.boot_state.next_frame.take() {
                    state.boot_state.video_width = width;
                    state.boot_state.video_height = height;
                    
                    if state.boot_state.video_texture.is_none() {
                        if let Ok(tex) = Texture::from_data(
                            ctx, 
                            width as i32, 
                            height as i32, 
                            TextureFormat::Rgba8, 
                            &rgba
                        ) {
                            state.boot_state.video_texture = Some(tex);
                        }
                    } else if let Some(tex) = &mut state.boot_state.video_texture {
                        let _ = tex.set_data(
                            ctx, 
                            0, 
                            0, 
                            width as i32, 
                            height as i32, 
                            &rgba
                        );
                    }
                    
                    // Recycle the buffer
                    if let Some(tx) = &state.boot_state.recycler_tx {
                        let _ = tx.send(rgba);
                    }
                }
                state.boot_state.video_timer -= frame_duration;
            }
        } else if state.boot_state.video_ended {
             println!("Video ended. Looping...");
             // Loop the video
             state.boot_state.video_receiver = None;
             state.boot_state.recycler_tx = None;
             state.boot_state.video_ended = false;
             state.boot_state.video_timer = 0.0;
             
             // Stop previous sound instance if any
             if let Some(instance) = &mut state.boot_state.video_instance {
                 instance.stop();
                 println!("Stopped previous audio instance.");
             }
             state.boot_state.video_instance = None;
             
             // Re-init will happen in next frame because video_receiver is None
        }
    }
    
    // Asset Loading
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
        
        if input::is_key_pressed(ctx, Key::Enter) {
             if let Some(instance) = &mut state.boot_state.video_instance {
                 instance.stop();
             }
             state.scene = Scene::Menu;
        }
    }

    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    graphics::clear(ctx, Color::BLACK);

    // Draw Video
    if let Some(tex) = &state.boot_state.video_texture {
        // Scale to fit screen
        let scale_x = SCREEN_WIDTH as f32 / state.boot_state.video_width as f32;
        let scale_y = SCREEN_HEIGHT as f32 / state.boot_state.video_height as f32;
        
        tex.draw(ctx, tetra::graphics::DrawParams::new()
            .scale(Vec2::new(scale_x, scale_y)));
    } else {
        // Fallback text if video failed
        let text = "Loading...";
        let mut t = Text::new(text, state.font.clone());
        t.draw(ctx, Vec2::new(10.0, 10.0));
    }

    // Draw Loading Progress (Small in corner)
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
            10.0,
        );
        // Draw with black outline for visibility
        text.draw(ctx, tetra::graphics::DrawParams::new().position(pos + Vec2::new(1.0, 1.0)).color(Color::BLACK));
        text.draw(ctx, pos);
    }

    // Draw "Press Enter"
    if state.boot_state.waiting_for_input {
        let msg = "Press Enter to continue";
        let mut text = Text::new(msg, state.font.clone());
        let bounds = text.get_bounds(ctx).unwrap();
        let pos = Vec2::new(
            SCREEN_WIDTH as f32 - bounds.width - 20.0,
            SCREEN_HEIGHT as f32 - bounds.height - 20.0,
        );
        text.draw(ctx, tetra::graphics::DrawParams::new().position(pos + Vec2::new(1.0, 1.0)).color(Color::BLACK));
        text.draw(ctx, pos);
    }

    Ok(())
}
