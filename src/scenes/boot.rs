use tetra::Context;
use tetra::graphics::{Color, DrawParams};
use tetra::graphics::text::Text;
use tetra::math::Vec2;
use crate::game_state::GameState;
use crate::defs::Scene;

pub struct BootState {
    pub boot_lines: Vec<String>,
    pub boot_text_cache: Vec<Option<(Text, Option<Text>)>>,
    pub current_line: usize,
    pub current_char: usize,
    pub char_timer: f32,
    pub boot_complete_timer: f32,
    pub loading_step: usize,
    pub loading_substep: usize,
    pub spinner_index: usize,
    pub spinner_direction: i32,
    pub spinner_timer: f32,
}

impl BootState {
    pub fn new() -> Self {
        Self {
            boot_lines: Vec::new(),
            boot_text_cache: Vec::new(),
            current_line: 0,
            current_char: 0,
            char_timer: 0.0,
            boot_complete_timer: 0.0,
            loading_step: 0,
            loading_substep: 0,
            spinner_index: 0,
            spinner_direction: 1,
            spinner_timer: 0.0,
        }
    }
}

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    // Ensure cache is synced with lines (handle initial line)
    while state.boot_state.boot_text_cache.len() < state.boot_state.boot_lines.len() {
            state.boot_state.boot_text_cache.push(None);
    }
    for i in 0..state.boot_state.boot_lines.len() {
        if state.boot_state.boot_text_cache[i].is_none() {
            let line = &state.boot_state.boot_lines[i];
            let cached = if line.starts_with("[  OK  ]") {
                let ok_part = Text::new("[  OK  ]", state.font.clone());
                let rest = Text::new(&line[8..], state.font.clone());
                Some((ok_part, Some(rest)))
            } else if line.starts_with("[ .... ]") {
                let wait_part = Text::new("[ .... ]", state.font.clone());
                let rest = Text::new(&line[8..], state.font.clone());
                Some((wait_part, Some(rest)))
            } else if line.starts_with("[ ") && line.len() >= 8 && line.chars().nth(7) == Some(']') {
                let spinner_part = Text::new(&line[0..8], state.font.clone());
                let rest = Text::new(&line[8..], state.font.clone());
                Some((spinner_part, Some(rest)))
            } else {
                let text = Text::new(line, state.font.clone());
                Some((text, None))
            };
            state.boot_state.boot_text_cache[i] = cached;
        }
    }

    state.boot_state.char_timer += 1.0;
    
    // Animate Spinner
    if state.boot_state.loading_substep == 1 {
        state.boot_state.spinner_timer += 1.0;
        if state.boot_state.spinner_timer > 5.0 { // Speed of animation
            state.boot_state.spinner_timer = 0.0;
            
            // Update index
            if state.boot_state.spinner_direction == 1 {
                state.boot_state.spinner_index += 1;
                if state.boot_state.spinner_index >= 3 {
                    state.boot_state.spinner_direction = -1;
                }
            } else {
                if state.boot_state.spinner_index > 0 {
                    state.boot_state.spinner_index -= 1;
                } else {
                    state.boot_state.spinner_direction = 1;
                    state.boot_state.spinner_index = 1;
                }
            }

            // Update the last line text
            if !state.boot_state.boot_lines.is_empty() {
                let last_idx = state.boot_state.boot_lines.len() - 1;
                let line = &state.boot_state.boot_lines[last_idx];
                if line.contains("Loading asset:") {
                    let parts: Vec<&str> = line.split("Loading asset:").collect();
                    if parts.len() > 1 {
                        let asset_name = parts[1];
                        let mut spinner_str = String::from("[ ");
                        for i in 0..4 {
                            if i == state.boot_state.spinner_index {
                                spinner_str.push('*');
                            } else {
                                spinner_str.push(' ');
                            }
                        }
                        spinner_str.push_str(" ]");
                        
                        state.boot_state.boot_lines[last_idx] = format!("{} Loading asset:{}", spinner_str, asset_name);
                        
                        // Update cache immediately to prevent blinking
                        // Cache the FULL spinner string in part1 to ensure correct width calculation in draw
                        let spinner_part = Text::new(&spinner_str, state.font.clone());
                        let rest = Text::new(&state.boot_state.boot_lines[last_idx][8..], state.font.clone());
                        state.boot_state.boot_text_cache[last_idx] = Some((spinner_part, Some(rest)));
                    }
                }
            }
        }
    }

    // Load one asset every few frames
    if state.boot_state.char_timer > 3.0 {
        match state.boot_state.loading_substep {
            0 => {
                // Step 0: Print "Loading..."
                let asset_count = crate::assets::ASSET_LIST.len();
                let asset_name = if state.boot_state.loading_step < asset_count {
                    crate::assets::ASSET_LIST[state.boot_state.loading_step].name
                } else if state.boot_state.loading_step == asset_count {
                    "System"
                } else {
                    ""
                };

                if !asset_name.is_empty() && state.boot_state.loading_step < asset_count {
                    if state.is_asset_loaded(state.boot_state.loading_step) {
                            // Skip spinner if already loaded
                            state.boot_state.boot_lines.push(format!("[  OK  ] Loaded asset: {}", asset_name));
                            state.boot_state.boot_text_cache.push(None);
                            state.boot_state.current_line = state.boot_state.boot_lines.len();
                            state.boot_state.loading_step += 1;
                            state.boot_state.char_timer = 0.0;
                    } else {
                            state.boot_state.boot_lines.push(format!("[ *    ] Loading asset: {}", asset_name));
                            state.boot_state.boot_text_cache.push(None);
                            state.boot_state.current_line = state.boot_state.boot_lines.len();
                            state.boot_state.loading_substep = 1;
                            state.boot_state.char_timer = 0.0;
                            state.boot_state.spinner_index = 0;
                            state.boot_state.spinner_direction = 1;
                    }
                } else if state.boot_state.loading_step == asset_count {
                    // Final step
                    state.boot_state.boot_lines.push("Welcome to VibeCoded Linux 1.0 LTS (tty1)".to_string());
                    state.boot_state.boot_text_cache.push(None);
                    state.boot_state.current_line = state.boot_state.boot_lines.len();
                    state.boot_state.loading_step += 1;
                    state.boot_state.char_timer = 0.0;
                } else if state.boot_state.loading_step == asset_count + 1 {
                    state.boot_state.boot_complete_timer += 1.0;
                    if state.boot_state.boot_complete_timer > 60.0 {
                        state.scene = Scene::Menu;
                    }
                }
            }
            1 => {
                // Step 1: Perform Load and Update Text
                let already_loaded = state.is_asset_loaded(state.boot_state.loading_step);
                
                // Simulate delay for animation to be visible ONLY if not already loaded
                if !already_loaded && state.boot_state.char_timer < 20.0 { // Wait 20 frames to show animation
                    return Ok(());
                }

                let mut loaded_text = String::new();
                if state.boot_state.loading_step < crate::assets::ASSET_LIST.len() {
                    let name = crate::assets::ASSET_LIST[state.boot_state.loading_step].name;
                    if !already_loaded {
                        let asset = crate::assets::load_asset_by_index(ctx, state.boot_state.loading_step)?;
                        state.assign_asset(state.boot_state.loading_step, asset);
                    }
                    loaded_text = format!("[  OK  ] Loaded asset: {}", name);
                }

                if !loaded_text.is_empty() {
                    let last_idx = state.boot_state.boot_lines.len() - 1;
                    state.boot_state.boot_lines[last_idx] = loaded_text;
                    
                    // Update cache immediately to prevent blinking
                    let line = &state.boot_state.boot_lines[last_idx];
                    let ok_part = Text::new("[  OK  ]", state.font.clone());
                    let rest = Text::new(&line[8..], state.font.clone());
                    state.boot_state.boot_text_cache[last_idx] = Some((ok_part, Some(rest)));
                    
                    state.boot_state.loading_step += 1;
                    state.boot_state.loading_substep = 0;
                    state.boot_state.char_timer = 0.0;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    let mut y = 20.0;
    // Only draw last 25 lines to simulate scrolling if needed, but for now just draw all
    // Or better, draw from current_line - 25
    let start_line = if state.boot_state.current_line > 25 { state.boot_state.current_line - 25 } else { 0 };
    
    for i in start_line..state.boot_state.boot_lines.len() {
        let line = &state.boot_state.boot_lines[i];
        if i < state.boot_state.current_line {
            // Use cache
            if let Some((part1, part2)) = &mut state.boot_state.boot_text_cache[i] {
                if line.starts_with("[  OK  ]") {
                    part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::GREEN));
                    if let Some(p2) = part2 {
                        let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                        p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                    }
                } else if line.starts_with("[ ") && line.len() >= 8 && line.chars().nth(7) == Some(']') {
                    // Spinner - Render full block in Yellow to preserve spacing
                    part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::rgb(1.0, 1.0, 0.0)));
                    
                    if let Some(p2) = part2 {
                        let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                        p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                    }
                } else if line.starts_with("[ WARN ]") {
                    part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::rgb(1.0, 0.5, 0.0)));
                    if let Some(p2) = part2 {
                        let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                        p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                    }
                } else if line.starts_with("[ FAILED ]") {
                    part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::RED));
                    if let Some(p2) = part2 {
                        let w = part1.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                        p2.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                    }
                } else {
                    part1.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                }
            }
        } else if i == state.boot_state.current_line {
            // Determine parts
            let (prefix_str, prefix_color, text_content) = if line.starts_with("[  OK  ]") {
                (Some("[  OK  ]"), Some(Color::GREEN), &line[8..])
            } else if line.starts_with("[ .... ]") {
                (Some("[ .... ]"), Some(Color::WHITE), &line[8..])
            } else if line.starts_with("[ WARN ]") {
                (Some("[ WARN ]"), Some(Color::rgb(1.0, 0.5, 0.0)), &line[8..])
            } else if line.starts_with("[ FAILED ]") {
                (Some("[ FAILED ]"), Some(Color::RED), &line[10..])
            } else if line.starts_with("[ ") && line.len() >= 8 && line.chars().nth(7) == Some(']') {
                // This is likely our spinner or a custom status
                (Some(&line[0..8]), Some(Color::rgb(1.0, 1.0, 0.0)), &line[8..])
            } else {
                (None, None, line.as_str())
            };

            if state.boot_state.current_char == 1 {
                // Show text only, indented
                let indent = if prefix_str.is_some() {
                    // Approx width of "[  OK  ]" (8 chars) * char width (approx 10px?)
                    // Better to measure a dummy text
                    let mut dummy = Text::new("[  OK  ]", state.font.clone());
                    dummy.get_bounds(ctx).map(|b| b.width).unwrap_or(80.0)
                } else {
                    0.0
                };
                
                let mut t = Text::new(text_content, state.font.clone());
                t.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + indent, y)).color(Color::WHITE));
            } else if state.boot_state.current_char == 2 {
                // Show full
                if let (Some(p_str), Some(p_col)) = (prefix_str, prefix_color) {
                    let mut p_text = Text::new(p_str, state.font.clone());
                    p_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(p_col));
                    
                    // Use fixed width based on standard prefix to prevent jittering during animation
                    let mut dummy = Text::new("[  OK  ]", state.font.clone());
                    let w = dummy.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);

                    let mut t = Text::new(text_content, state.font.clone());
                    t.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + w, y)).color(Color::WHITE));
                } else {
                    let mut t = Text::new(text_content, state.font.clone());
                    t.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                }
            }
        }
        y += 20.0;
    }
    Ok(())
}
