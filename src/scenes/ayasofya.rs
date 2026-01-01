use tetra::Context;
use tetra::graphics::{self, Color, DrawParams};
use tetra::input::{self, Key};
use tetra::math::Vec2;

use crate::game_state::GameState;
use crate::defs::{Scene, SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let speed = 2.0;
    
    // Movement (Simple left/right/up/down)
    if input::is_key_down(ctx, Key::W) || input::is_key_down(ctx, Key::Up) {
        state.player_pos.y -= speed;
    }
    if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
        state.player_pos.y += speed;
    }
    if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
        state.player_pos.x -= speed;
    }
    if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
        state.player_pos.x += speed;
    }

    // Boundaries
    if state.player_pos.y < 150.0 { state.player_pos.y = 150.0; }
    if state.player_pos.y > SCREEN_HEIGHT as f32 - 50.0 { state.player_pos.y = SCREEN_HEIGHT as f32 - 50.0; }
    // Removed right boundary clamp to allow exit

    // Exit Logic (Left side)
    if state.player_pos.x < 0.0 {
        state.scene = Scene::Desktop;
        state.current_stage = 3;
        state.player_pos.x = 400.0; // Center of stage 3 (entrance)
        state.player_pos.y = 400.0; // Below the door
        state.mosque_outfit = 0;
    }

    // Exit Logic (Right side)
    if state.player_pos.x > SCREEN_WIDTH as f32 {
        state.scene = Scene::Desktop;
        state.current_stage = 3;
        state.player_pos.x = 400.0; // Center of stage 3 (entrance)
        state.player_pos.y = 400.0; // Below the door
        state.mosque_outfit = 0;
    }

    // Ensure music is off
    if state.music_playing {
        if let Some(instance) = &mut state.music_instance {
            instance.stop();
        }
        state.music_playing = false;
    }

    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    graphics::clear(ctx, Color::BLACK);

    if let Some(texture) = &state.ayasofya_ici_texture {
        // Draw Background
        let bg_width = texture.width() as f32;
        let bg_height = texture.height() as f32;
        let scale_x = SCREEN_WIDTH as f32 / bg_width;
        let scale_y = SCREEN_HEIGHT as f32 / bg_height;
        
        texture.draw(ctx, DrawParams::new()
            .position(Vec2::new(0.0, 0.0))
            .scale(Vec2::new(scale_x, scale_y))
        );
    }

    // Draw Player
    let player_texture = match state.mosque_outfit {
        1 => &state.player_texture_fes,
        2 => &state.player_texture_takke,
        _ => &state.player_texture_front,
    };

    if let Some(texture) = player_texture {
        let width = texture.width() as f32;
        let height = texture.height() as f32;
        let origin = Vec2::new(width / 2.0, height / 2.0);
        
        texture.draw(ctx, DrawParams::new()
            .position(state.player_pos)
            .origin(origin)
            .scale(Vec2::new(4.0, 4.0))
        );
    }

    Ok(())
}
