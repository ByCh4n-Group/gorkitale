use tetra::Context;
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::graphics::text::Text;
use tetra::math::Vec2;
use rand::Rng;
use crate::game_state::GameState;
use crate::defs::{SCREEN_WIDTH, SCREEN_HEIGHT, Scene};

#[derive(PartialEq, Clone, Copy)]
pub enum MenuSubState {
    Main,
    SaveSelect,
    CreateSave,
    Settings,
    Credits,
}

pub struct SnowParticle {
    pub pos: Vec2<f32>,
    pub speed: f32,
    pub size: f32,
}

pub struct MenuState {
    pub sub_state: MenuSubState,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub snow_particles: Vec<SnowParticle>,
    pub title_blink_timers: Vec<f32>,
    pub input_buffer: String,
    pub error_message: Option<String>,
    
    // Chase Animation
    pub chara_pos: Vec2<f32>,
    pub sans_pos: Vec2<f32>,
    pub is_chara_chasing: bool, // true = Chara chases Sans, false = Sans chases Chara
}

impl MenuState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut snow_particles = Vec::new();
        for _ in 0..100 {
            snow_particles.push(SnowParticle {
                pos: Vec2::new(
                    rng.gen_range(0.0..SCREEN_WIDTH as f32),
                    rng.gen_range(0.0..SCREEN_HEIGHT as f32),
                ),
                speed: rng.gen_range(0.5..2.0),
                size: rng.gen_range(1.0..3.0),
            });
        }

        let title_len = "Gorkitale".len();
        let mut title_blink_timers = Vec::new();
        for _ in 0..title_len {
            title_blink_timers.push(rng.gen_range(0.0..1.0));
        }

        Self {
            sub_state: MenuSubState::Main,
            options: vec![
                "Start Game".to_string(),
                "Create Save".to_string(),
                "Select Save".to_string(),
                "Settings".to_string(),
                "Credits".to_string(),
                "Quit Game".to_string(),
            ],
            selected_index: 0,
            snow_particles,
            title_blink_timers,
            input_buffer: String::new(),
            error_message: None,
            
            chara_pos: Vec2::new(-100.0, 500.0),
            sans_pos: Vec2::new(100.0, 500.0),
            is_chara_chasing: true,
        }
    }
}

pub fn update(_ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Update Snow
    let mut rng = rand::thread_rng();
    for particle in &mut state.menu_state.snow_particles {
        particle.pos.y += particle.speed;
        if particle.pos.y > SCREEN_HEIGHT as f32 {
            particle.pos.y = -10.0;
            particle.pos.x = rng.gen_range(0.0..SCREEN_WIDTH as f32);
        }
    }

    // Update Title Blink
    for timer in &mut state.menu_state.title_blink_timers {
        *timer -= 0.02; // Speed of blink
        if *timer <= 0.0 {
            *timer = rng.gen_range(0.1..1.5); // Random reset
        }
    }

    // Update Chase Animation
    let speed = 3.0;
    if state.menu_state.is_chara_chasing {
        // Chara chases Sans (Left to Right)
        state.menu_state.sans_pos.x += speed;
        state.menu_state.chara_pos.x += speed;
        
        if state.menu_state.chara_pos.x > SCREEN_WIDTH as f32 + 100.0 {
            // Reset for Sans chasing Chara (Right to Left)
            state.menu_state.is_chara_chasing = false;
            state.menu_state.chara_pos = Vec2::new(SCREEN_WIDTH as f32 + 100.0, 500.0);
            state.menu_state.sans_pos = Vec2::new(SCREEN_WIDTH as f32 + 300.0, 500.0);
        }
    } else {
        // Sans chases Chara (Right to Left)
        state.menu_state.chara_pos.x -= speed;
        state.menu_state.sans_pos.x -= speed;
        
        if state.menu_state.sans_pos.x < -100.0 {
            // Reset for Chara chasing Sans (Left to Right)
            state.menu_state.is_chara_chasing = true;
            state.menu_state.sans_pos = Vec2::new(-100.0, 500.0);
            state.menu_state.chara_pos = Vec2::new(-300.0, 500.0);
        }
    }

    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Draw Snow
    for particle in &state.menu_state.snow_particles {
        let rect = Rectangle::new(particle.pos.x, particle.pos.y, particle.size, particle.size);
        let mesh = tetra::graphics::mesh::Mesh::rectangle(ctx, tetra::graphics::mesh::ShapeStyle::Fill, rect)?;
        mesh.draw(ctx, DrawParams::new().color(Color::WHITE));
    }

    // Draw Chase Animation (Background)
    // We need textures for Chara and Sans. Assuming they are loaded in GameState.
    // If not, we might need to use placeholders or ensure they are loaded.
    // Based on assets.rs, index 0 is Player Front (Chara?), index 8 is Sans.
    // Let's check if they are loaded.
    
    let chara_texture = if state.is_asset_loaded(0) {
        state.player.texture_front.clone()
    } else { None };

    let sans_texture = if state.is_asset_loaded(8) {
        state.world.sans_texture.clone()
    } else { None };

    if let (Some(chara), Some(sans)) = (chara_texture, sans_texture) {
        // Draw Chara
        let chara_scale = if state.menu_state.is_chara_chasing { Vec2::new(2.0, 2.0) } else { Vec2::new(-2.0, 2.0) }; // Flip if running left
        chara.draw(ctx, DrawParams::new()
            .position(state.menu_state.chara_pos)
            .origin(Vec2::new(chara.width() as f32 / 2.0, chara.height() as f32 / 2.0))
            .scale(chara_scale)
            .color(Color::rgba(1.0, 1.0, 1.0, 0.5)) // Semi-transparent for background effect
        );

        // Draw Sans
        let sans_scale = if state.menu_state.is_chara_chasing { Vec2::new(2.0, 2.0) } else { Vec2::new(-2.0, 2.0) };
        sans.draw(ctx, DrawParams::new()
            .position(state.menu_state.sans_pos)
            .origin(Vec2::new(sans.width() as f32 / 2.0, sans.height() as f32 / 2.0))
            .scale(sans_scale)
            .color(Color::rgba(1.0, 1.0, 1.0, 0.5))
        );
    }

    match state.menu_state.sub_state {
        MenuSubState::Main => draw_main_menu(ctx, state),
        MenuSubState::SaveSelect => draw_save_select(ctx, state),
        MenuSubState::CreateSave => draw_create_save(ctx, state),
        MenuSubState::Settings => draw_settings(ctx, state),
        MenuSubState::Credits => draw_credits(ctx, state),
    }?;

    // Draw Transition Fade
    if state.scene == Scene::TransitionToDesktop {
        let alpha = (state.transition_timer / 120.0).min(1.0);
        let fade_rect = tetra::graphics::mesh::Mesh::rectangle(
            ctx, 
            tetra::graphics::mesh::ShapeStyle::Fill, 
            Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
        )?;
        fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, alpha)));
    }

    Ok(())
}

fn draw_main_menu(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Draw Title "Gorkitale"
    let title = "Gorkitale";
    let start_x = (SCREEN_WIDTH as f32 / 2.0) - (title.len() as f32 * 20.0); // Approx centering
    let start_y = 100.0;

    for (i, char) in title.chars().enumerate() {
        let timer = state.menu_state.title_blink_timers[i];
        let alpha = if timer > 0.2 { 1.0 } else { 0.3 }; // Blink effect
        
        let mut text = Text::new(char.to_string(), state.font.clone());
        let pos = Vec2::new(start_x + (i as f32 * 40.0), start_y);
        let color = Color::rgba(1.0, 1.0, 1.0, alpha);
        
        // Simulate Bold by drawing multiple times with slight offsets
        for offset_x in 0..=1 {
            for offset_y in 0..=1 {
                text.draw(ctx, DrawParams::new()
                    .position(pos + Vec2::new(offset_x as f32, offset_y as f32))
                    .scale(Vec2::new(2.0, 2.0))
                    .color(color)
                );
            }
        }
    }

    // Draw Current User Info
    if let Some(user) = state.system.users.first() {
        let user_text = format!("Current Profile: {}", user.username);
        let mut text = Text::new(user_text, state.font.clone());
        text.draw(ctx, DrawParams::new()
            .position(Vec2::new(20.0, 20.0))
            .color(Color::rgb(0.7, 0.7, 0.7))
        );
    } else {
        let mut text = Text::new("No Profile Selected", state.font.clone());
        text.draw(ctx, DrawParams::new()
            .position(Vec2::new(20.0, 20.0))
            .color(Color::rgb(0.7, 0.7, 0.7))
        );
    }

    // Draw Options
    let menu_start_y = (SCREEN_HEIGHT as f32 / 2.0) - (state.menu_state.options.len() as f32 * 20.0); // Center vertically
    let menu_start_x = (SCREEN_WIDTH as f32 / 2.0) - 100.0; // Fixed X position for left alignment, slightly offset from center

    for (i, option) in state.menu_state.options.iter().enumerate() {
        let color = if i == state.menu_state.selected_index {
            Color::rgb(1.0, 1.0, 0.0)
        } else {
            Color::WHITE
        };

        let prefix = if i == state.menu_state.selected_index { "> " } else { "  " };
        let mut text = Text::new(format!("{}{}", prefix, option), state.font.clone());
        
        // Left aligned at fixed X
        text.draw(ctx, DrawParams::new()
            .position(Vec2::new(menu_start_x, menu_start_y + (i as f32 * 40.0)))
            .color(color)
        );
    }

    Ok(())
}

fn draw_save_select(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Select Profile", state.font.clone());
    title.draw(ctx, DrawParams::new().position(Vec2::new(300.0, 50.0)).scale(Vec2::new(1.5, 1.5)));

    let start_y = 150.0;
    
    // List users
    for (i, user) in state.system.users.iter().enumerate() {
        let color = if i == state.menu_state.selected_index {
            Color::rgb(1.0, 1.0, 0.0)
        } else {
            Color::WHITE
        };
        
        let prefix = if i == state.menu_state.selected_index { "> " } else { "  " };
        let mut text = Text::new(format!("{}{}", prefix, user.username), state.font.clone());
        text.draw(ctx, DrawParams::new()
            .position(Vec2::new(200.0, start_y + (i as f32 * 30.0)))
            .color(color)
        );
    }

    // "Press Esc to go back"
    let mut hint = Text::new("Press Esc to go back", state.font.clone());
    hint.draw(ctx, DrawParams::new().position(Vec2::new(200.0, 500.0)).color(Color::rgb(0.5, 0.5, 0.5)));

    Ok(())
}

fn draw_create_save(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Create New Profile", state.font.clone());
    title.draw(ctx, DrawParams::new().position(Vec2::new(250.0, 100.0)).scale(Vec2::new(1.5, 1.5)));

    let mut prompt = Text::new("Enter Name:", state.font.clone());
    prompt.draw(ctx, DrawParams::new().position(Vec2::new(250.0, 200.0)));

    // Handle text wrapping/scrolling
    let display_text = if state.menu_state.input_buffer.len() > 20 {
        let start = state.menu_state.input_buffer.len() - 20;
        format!("...{}_", &state.menu_state.input_buffer[start..])
    } else {
        format!("{}_", state.menu_state.input_buffer)
    };

    let mut input = Text::new(display_text, state.font.clone());
    input.draw(ctx, DrawParams::new().position(Vec2::new(250.0, 240.0)).color(Color::rgb(1.0, 1.0, 0.0)));

    if let Some(err) = &state.menu_state.error_message {
        let mut err_text = Text::new(err, state.font.clone());
        err_text.draw(ctx, DrawParams::new().position(Vec2::new(250.0, 300.0)).color(Color::RED));
    }

    let mut hint = Text::new("Press Enter to Confirm, Esc to Cancel", state.font.clone());
    hint.draw(ctx, DrawParams::new().position(Vec2::new(200.0, 500.0)).color(Color::rgb(0.5, 0.5, 0.5)));

    Ok(())
}

fn draw_settings(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Settings", state.font.clone());
    title.draw(ctx, DrawParams::new().position(Vec2::new(300.0, 50.0)).scale(Vec2::new(1.5, 1.5)));

    let lang_text = format!("Language: {:?}", state.system.language);
    let mut text = Text::new(lang_text, state.font.clone());
    text.draw(ctx, DrawParams::new().position(Vec2::new(200.0, 200.0)));

    let mut hint = Text::new("Press Esc to go back", state.font.clone());
    hint.draw(ctx, DrawParams::new().position(Vec2::new(200.0, 500.0)).color(Color::rgb(0.5, 0.5, 0.5)));

    Ok(())
}

fn draw_credits(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut title = Text::new("Credits", state.font.clone());
    title.draw(ctx, DrawParams::new().position(Vec2::new(300.0, 50.0)).scale(Vec2::new(1.5, 1.5)));

    let credits = vec![
        "Developed by: VibeCoded",
        "Engine: Tetra (Rust)",
        "Art: ...",
        "Music: ...",
    ];

    for (i, line) in credits.iter().enumerate() {
        let mut text = Text::new(*line, state.font.clone());
        text.draw(ctx, DrawParams::new().position(Vec2::new(200.0, 150.0 + (i as f32 * 30.0))));
    }

    let mut hint = Text::new("Press Esc to go back", state.font.clone());
    hint.draw(ctx, DrawParams::new().position(Vec2::new(200.0, 500.0)).color(Color::rgb(0.5, 0.5, 0.5)));

    Ok(())
}
