use tetra::Context;
use tetra::graphics::{self, Color, DrawParams, Rectangle};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::input::{self, Key};
use tetra::math::{Vec2};
use rand::Rng;

use crate::game_state::GameState;
use crate::defs::{Scene, Direction, SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let speed = 2.0;
    let mut next_pos = state.player_pos;

    if input::is_key_down(ctx, Key::W) || input::is_key_down(ctx, Key::Up) {
        next_pos.y -= speed;
        state.player_direction = Direction::Front;
    }
    if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
        next_pos.y += speed;
        state.player_direction = Direction::Front;
    }
    if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
        next_pos.x -= speed;
        state.player_direction = Direction::Left;
    }
    if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
        next_pos.x += speed;
        state.player_direction = Direction::Right;
    }

    // Collision Check
    let mut collided = false;
    let player_radius = 20.0;

    // Screen Boundaries (Top/Bottom)
    if next_pos.y < 150.0 || next_pos.y > SCREEN_HEIGHT as f32 - 50.0 {
        collided = true;
    }

    // Object Collision
    if state.current_stage == 1 {
        // Sans
        let dx = next_pos.x - state.sans_pos.x;
        let dy = next_pos.y - state.sans_pos.y;
        if (dx*dx + dy*dy).sqrt() < (player_radius + 50.0) {
            collided = true;
        }
        
        // MusicBox
        let dx = next_pos.x - state.musicbox_pos.x;
        let dy = next_pos.y - state.musicbox_pos.y;
        if (dx*dx + dy*dy).sqrt() < (player_radius + 40.0) {
            collided = true;
        }
    } else if state.current_stage == 2 {
        // Rarity
        if state.rarity_alive {
            let dx = next_pos.x - state.rarity_pos.x;
            let dy = next_pos.y - state.rarity_pos.y;
            if (dx*dx + dy*dy).sqrt() < (player_radius + 50.0) {
                collided = true;
            }
        }
    } else if state.current_stage == 3 {
        // Ayasofya (No collision for now, maybe walls later)
    } else if state.current_stage == 4 {
        // Eilish
        let dx = next_pos.x - state.eilish_pos.x;
        let dy = next_pos.y - state.eilish_pos.y;
        if (dx*dx + dy*dy).sqrt() < (player_radius + 50.0) {
            collided = true;
        }
    }

    if !collided {
        state.player_pos = next_pos;
    }

    // Stage Transition Logic
    if state.player_pos.x > SCREEN_WIDTH as f32 || (state.player_pos.x < 0.0 && state.current_stage > 1) {
        if !state.fade_out {
            state.fade_out = true;
        }
    } else if state.player_pos.x < 0.0 {
        state.player_pos.x = 0.0;
    }

    if state.fade_out {
        state.fade_alpha += 0.05;
        if state.fade_alpha >= 1.0 {
            state.fade_alpha = 1.0;
            if state.player_pos.x > SCREEN_WIDTH as f32 {
                state.current_stage += 1;
                if state.current_stage > 4 {
                    state.current_stage = 1;
                }
                state.player_pos.x = 10.0;
            } else {
                state.current_stage -= 1;
                state.player_pos.x = SCREEN_WIDTH as f32 - 10.0;
            }
            state.fade_out = false;
        }
    } else if state.fade_alpha > 0.0 {
        state.fade_alpha -= 0.05;
    }

    // Dead Space Logic (Stage 4, Right Side)
    if state.current_stage == 4 && state.player_pos.x > 500.0 {
        state.player_health -= 0.5; // Damage multiplier
        
        if state.player_health <= 0.0 {
            // Game Over -> Kernel Panic
            state.generate_kernel_panic();
            state.scene = Scene::KernelPanic;
            state.session_started = false;
        }
    }

    // MusicBox Interaction (Stage 1)
    if state.current_stage == 1 {
        let dx = state.player_pos.x - state.musicbox_pos.x;
        let dy = state.player_pos.y - state.musicbox_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 120.0 {
            if input::is_key_pressed(ctx, Key::F) {
                if state.music_playing {
                    if let Some(instance) = &mut state.music_instance {
                        instance.stop();
                    }
                    state.music_playing = false;
                } else {
                    if let Some(track) = &state.music_track {
                        if let Ok(instance) = track.play(ctx) {
                            instance.set_repeating(true);
                            state.music_instance = Some(instance);
                            state.music_playing = true;
                        }
                    }
                }
            }
        }
    }

    if state.music_playing {
        state.disco_timer += 1.0;
        if state.disco_timer > 10.0 {
            state.disco_timer = 0.0;
            let mut rng = rand::thread_rng();
            state.disco_color = Color::rgb(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            );
        }
    }

    // Sans Interaction (Stage 1)
    if state.current_stage == 1 {
        let dx = state.player_pos.x - state.sans_pos.x;
        let dy = state.player_pos.y - state.sans_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 120.0 {
            if input::is_key_pressed(ctx, Key::F) {
                state.scene = Scene::CombatTransition;
                state.fade_out = true;
                state.fade_alpha = 0.0;
            }
        }
    }

    // Gaster Interaction (Stage 2)
    if state.current_stage == 2 {
        // Simple distance check
        let dx = state.player_pos.x - state.gaster_pos.x;
        let dy = state.player_pos.y - state.gaster_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 120.0 {
            if input::is_key_pressed(ctx, Key::F) {
                state.gaster_talking = !state.gaster_talking;
                if state.gaster_talking {
                    let mut rng = rand::thread_rng();
                    let idx = rng.gen_range(0..state.gaster_dialogues.len());
                    state.current_gaster_dialogue = state.gaster_dialogues[idx].clone();
                }
            }
        } else {
            if state.gaster_talking {
                state.gaster_talking = false;
            }
        }

        // Rarity Interaction (Stage 2)
        if state.rarity_alive {
            let dx = state.player_pos.x - state.rarity_pos.x;
            let dy = state.player_pos.y - state.rarity_pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Only interact if behind (Player X < Rarity X) and close
            if distance < 120.0 && state.player_pos.x < state.rarity_pos.x {
                if input::is_key_pressed(ctx, Key::F) {
                    state.rarity_alive = false;
                    state.rarity_stabbed_timer = 180.0; // 3 seconds
                }
            }
        } else if state.rarity_stabbed_timer > 0.0 {
            state.rarity_stabbed_timer -= 1.0;
        }
    }

    // Ayasofya Interaction (Stage 3)
    if state.current_stage == 3 {
        // Door area: Expanded range based on user feedback
        // Massive range to ensure it's easy to enter
        let door_rect = Rectangle::new(200.0, 200.0, 400.0, 400.0);
        if state.player_pos.x >= door_rect.x && state.player_pos.x <= door_rect.x + door_rect.width &&
           state.player_pos.y >= door_rect.y && state.player_pos.y <= door_rect.y + door_rect.height {
             if input::is_key_pressed(ctx, Key::F) {
                 state.scene = Scene::AyasofyaInside;
                 state.player_pos = Vec2::new(100.0, 300.0); // Entrance inside
                 
                 // Randomly select outfit
                 let mut rng = rand::thread_rng();
                 state.mosque_outfit = rng.gen_range(1..3); // 1 or 2
             }
        }
    }

    // Eilish Interaction (Stage 4)
    crate::scenes::eilish::update(ctx, state);
    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    graphics::clear(ctx, Color::BLACK);
    
    if state.current_stage == 3 {
        if let Some(texture) = &state.ayasofya_giris_texture {
            let bg_width = texture.width() as f32;
            let bg_height = texture.height() as f32;
            let scale_x = SCREEN_WIDTH as f32 / bg_width;
            let scale_y = SCREEN_HEIGHT as f32 / bg_height;
            
            texture.draw(ctx, DrawParams::new()
                .position(Vec2::new(0.0, 0.0))
                .scale(Vec2::new(scale_x, scale_y))
            );
        }
    } else {
        if let Some(bg_texture) = &state.bg_texture {
            let bg_width = bg_texture.width() as f32;
            let bg_height = bg_texture.height() as f32;
            let scale_x = SCREEN_WIDTH as f32 / bg_width;
            let scale_y = SCREEN_HEIGHT as f32 / bg_height;
            
            bg_texture.draw(ctx, DrawParams::new()
                .position(Vec2::new(0.0, 0.0))
                .scale(Vec2::new(scale_x, scale_y))
                .color(if state.current_stage == 1 { Color::WHITE } 
                       else if state.current_stage == 2 { Color::rgb(0.8, 0.8, 1.0) } // Blueish tint
                       else { Color::rgb(1.0, 0.8, 0.8) }) // Reddish tint
            );
        }
    }

    // Draw Gaster (Stage 2)
    if state.current_stage == 2 {
        let gaster_texture = if state.gaster_talking {
            &state.npc_gaster_talking
        } else {
            &state.npc_gaster_standing
        };
        
        if let Some(tex) = gaster_texture {
            let g_width = tex.width() as f32;
            let g_height = tex.height() as f32;
            let g_origin = Vec2::new(g_width / 2.0, g_height / 2.0);
            
            tex.draw(ctx, DrawParams::new()
                .position(state.gaster_pos)
                .origin(g_origin)
                .scale(Vec2::new(3.0, 3.0))
            );
        }

        // Interaction Prompt
        let dx = state.player_pos.x - state.gaster_pos.x;
        let dy = state.player_pos.y - state.gaster_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 100.0 && !state.gaster_talking {
            let prompt = "Press F to interact";
            let mut text = Text::new(prompt, state.font.clone());
            let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(100.0);
            
            text.draw(ctx, DrawParams::new()
                .position(Vec2::new(state.gaster_pos.x - width / 2.0, state.gaster_pos.y - 80.0))
                .color(Color::rgb(1.0, 1.0, 0.0))
            );
        }

        // Dialogue Box
        if state.gaster_talking {
            // Draw a box at the bottom
            if let Ok(box_rect) = Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(50.0, 450.0, 700.0, 130.0),
            ) {
                box_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, 0.8)));
            }
            
            if let Ok(border_rect) = Mesh::rectangle(
                ctx,
                ShapeStyle::Stroke(2.0),
                Rectangle::new(50.0, 450.0, 700.0, 130.0),
            ) {
                border_rect.draw(ctx, DrawParams::new().color(Color::WHITE));
            }

            let mut text = Text::new(&state.current_gaster_dialogue, state.font.clone());
            text.draw(ctx, DrawParams::new().position(Vec2::new(70.0, 470.0)).color(Color::WHITE));
        }
    }

    // Ayasofya Door Prompt (Stage 3)
    if state.current_stage == 3 {
        let door_rect = Rectangle::new(350.0, 250.0, 100.0, 100.0);
        if state.player_pos.x >= door_rect.x && state.player_pos.x <= door_rect.x + door_rect.width &&
           state.player_pos.y >= door_rect.y && state.player_pos.y <= door_rect.y + door_rect.height {
            let prompt = "Press F to enter Ayasofya";
            let mut text = Text::new(prompt, state.font.clone());
            let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(100.0);
            
            text.draw(ctx, DrawParams::new()
                .position(Vec2::new(400.0 - width / 2.0, 200.0))
                .color(Color::WHITE)
            );
        }
    }

    // Draw Dead Space (Stage 4)
    if state.current_stage == 4 {
        let dead_space_rect = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(500.0, 0.0, 300.0, SCREEN_HEIGHT as f32),
        )?;
        dead_space_rect.draw(ctx, DrawParams::new().color(Color::rgba(1.0, 0.0, 0.0, 0.3)));
    }
    
    crate::scenes::eilish::draw(ctx, state)?;
    
    // Draw player
    let texture_opt = match state.player_direction {
        Direction::Front => &state.player_texture_front,
        Direction::Left => &state.player_texture_left,
        Direction::Right => &state.player_texture_right,
    };
    
    if let Some(texture) = texture_opt {
        // Center the sprite on player_pos
        let width = texture.width() as f32;
        let height = texture.height() as f32;
        let origin = Vec2::new(width / 2.0, height / 2.0);
        
        // Scale up the character (e.g. 3x)
        texture.draw(ctx, DrawParams::new()
            .position(state.player_pos)
            .origin(origin)
            .scale(Vec2::new(3.0, 3.0))
        );
    }
    
    // Draw MusicBox (Stage 1)
    if state.current_stage == 1 {
        if let Some(musicbox_texture) = &state.musicbox_texture {
            let m_width = musicbox_texture.width() as f32;
            let m_height = musicbox_texture.height() as f32;
            let m_origin = Vec2::new(m_width / 2.0, m_height / 2.0);
            
            musicbox_texture.draw(ctx, DrawParams::new()
                .position(state.musicbox_pos)
                .origin(m_origin)
                .scale(Vec2::new(0.3, 0.3))
            );
        }

        // Interaction Prompt
        let dx = state.player_pos.x - state.musicbox_pos.x;
        let dy = state.player_pos.y - state.musicbox_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 120.0 {
            let prompt = if state.music_playing { "Press F to Stop Music" } else { "Press F to Play Music" };
            let mut text = Text::new(prompt, state.font.clone());
            let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(100.0);
            
            text.draw(ctx, DrawParams::new()
                .position(Vec2::new(state.musicbox_pos.x - width / 2.0, state.musicbox_pos.y - 60.0))
                .color(Color::rgb(0.0, 1.0, 1.0))
            );
        }
    }

    // Disco Lights Overlay
    if state.music_playing {
        let light_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
        light_rect.draw(ctx, DrawParams::new().color(state.disco_color.with_alpha(0.2)));
    }

    // Draw Stage Indicator
    let stage_text = format!("Stage: {}/4", state.current_stage);
    let mut text = Text::new(stage_text, state.font.clone());
    text.draw(ctx, DrawParams::new().position(Vec2::new(10.0, 10.0)).color(Color::WHITE));

    // Draw Health Bar (Top Right)
    let bar_width = 150.0;
    let bar_height = 15.0;
    let padding = 10.0;
    let bar_x = SCREEN_WIDTH as f32 - bar_width - padding;
    let bar_y = 10.0;

    let health_bar_bg = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(bar_x, bar_y, bar_width, bar_height))?;
    health_bar_bg.draw(ctx, DrawParams::new().color(Color::rgb(0.2, 0.2, 0.2)));
    
    let health_fill_width = (state.player_health / 100.0) * bar_width;
    if health_fill_width > 0.0 {
        let health_bar_fg = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(bar_x, bar_y, health_fill_width, bar_height))?;
        health_bar_fg.draw(ctx, DrawParams::new().color(Color::RED));
    }
    
    let hp_text = format!("HP: {:.0}%", state.player_health);
    let mut hp_display = Text::new(hp_text, state.font.clone());
    // Position text to the left of the bar or below? Let's put it inside/below
    // Or just to the left
    let hp_bounds = hp_display.get_bounds(ctx).unwrap();
    hp_display.draw(ctx, DrawParams::new().position(Vec2::new(bar_x - hp_bounds.width - 10.0, bar_y)).color(Color::WHITE));

    // Draw FPS
    let fps = tetra::time::get_fps(ctx);
    let fps_text = format!("FPS: {:.0}", fps);
    let mut fps_display = Text::new(fps_text, state.font.clone());
    fps_display.draw(ctx, DrawParams::new().position(Vec2::new(10.0, 30.0)).color(Color::rgb(1.0, 1.0, 0.0)));

    // Fade Overlay
    if state.fade_alpha > 0.0 {
        let fade_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
        fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, state.fade_alpha)));
    }

    // Draw Sans in Stage 1
    if state.current_stage == 1 {
        if let Some(sans_handshake_texture) = &state.sans_handshake_texture {
            let s_width = sans_handshake_texture.width() as f32;
            let s_height = sans_handshake_texture.height() as f32;
            let s_origin = Vec2::new(s_width / 2.0, s_height / 2.0);
            
            sans_handshake_texture.draw(ctx, DrawParams::new()
                .position(state.sans_pos)
                .origin(s_origin)
                .scale(Vec2::new(3.0, 3.0)) // Increased scale from 2.0 to 3.0
            );
        }

        // Interaction Prompt
        let dx = state.player_pos.x - state.sans_pos.x;
        let dy = state.player_pos.y - state.sans_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 120.0 {
            let prompt = "Press F to interact";
            let mut text = Text::new(prompt, state.font.clone());
            let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(100.0);
            
            text.draw(ctx, DrawParams::new()
                .position(Vec2::new(state.sans_pos.x - width / 2.0, state.sans_pos.y - 80.0))
                .color(Color::rgb(1.0, 1.0, 0.0))
            );
        }
    }

    // Draw Rarity in Stage 2
    if state.current_stage == 2 {
        if state.rarity_alive {
            if let Some(rarity_texture) = &state.rarity_texture {
                let r_width = rarity_texture.width() as f32;
                let r_height = rarity_texture.height() as f32;
                let r_origin = Vec2::new(r_width / 2.0, r_height / 2.0);
                
                rarity_texture.draw(ctx, DrawParams::new()
                    .position(state.rarity_pos)
                    .origin(r_origin)
                    .scale(Vec2::new(1.3, 1.3))
                );
            }

            // Interaction Prompt (Only from behind)
            let dx = state.player_pos.x - state.rarity_pos.x;
            let dy = state.player_pos.y - state.rarity_pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < 120.0 && state.player_pos.x < state.rarity_pos.x {
                let prompt = "Press F to Stab";
                let mut text = Text::new(prompt, state.font.clone());
                let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(100.0);
                
                text.draw(ctx, DrawParams::new()
                    .position(Vec2::new(state.rarity_pos.x - width / 2.0, state.rarity_pos.y - 80.0))
                    .color(Color::RED)
                );
            }
        } else if state.rarity_stabbed_timer > 0.0 {
            // Draw stabbed message
            let msg = "You stabbed Rarity from behind!";
            let mut text = Text::new(msg, state.font.clone());
            let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(200.0);
            
            text.draw(ctx, DrawParams::new()
                .position(Vec2::new(state.rarity_pos.x - width / 2.0, state.rarity_pos.y))
                .color(Color::RED)
            );
        }
    }

    // Fade Transition Overlay
    if state.fade_out {
        let fade_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
        fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, state.fade_alpha)));
    }

    Ok(())
}
