use tetra::Context;
use tetra::Event;
use tetra::input::Key;
use crate::game_state::GameState;
use crate::defs::{Scene, Direction};
use crate::system::User;
use crate::scenes::menu::MenuSubState;
use tetra::math::Vec2;

pub fn handle_event(ctx: &mut Context, state: &mut GameState, event: Event) {
    match event {
        Event::TextInput { text } => {
            handle_text_input(state, text);
        }
        Event::KeyPressed { key } => {
            handle_key_pressed(ctx, state, key);
        }
        _ => {}
    }
}

fn handle_text_input(state: &mut GameState, text: String) {
    match state.scene {
        Scene::Menu => {
            if state.menu_state.sub_state == MenuSubState::CreateSave {
                // Limit length to 32 chars
                if state.menu_state.input_buffer.len() < 32 {
                    // Filter for safe characters (Alphanumeric + Space + Underscore + Hyphen)
                    // Also explicitly block non-ASCII to prevent crashes with fonts that don't support them
                    if text.chars().all(|c| (c.is_alphanumeric() || c == ' ' || c == '_' || c == '-') && c.is_ascii()) {
                        state.menu_state.input_buffer.push_str(&text);
                    }
                }
            }
        }
        _ => {}
    }
}

fn handle_key_pressed(ctx: &mut Context, state: &mut GameState, key: Key) {
    match key {
        Key::Backspace => {
            match state.scene {
                Scene::Menu => {
                    if state.menu_state.sub_state == MenuSubState::CreateSave {
                        state.menu_state.input_buffer.pop();
                    }
                }
                _ => {}
            }
        }
        Key::Enter => {
            handle_enter_key(ctx, state);
        }
        Key::Escape => {
            handle_escape_key(state);
        }
        Key::Up => {
            if state.scene == Scene::Menu {
                match state.menu_state.sub_state {
                    MenuSubState::Main => {
                        if state.menu_state.selected_index > 0 {
                            state.menu_state.selected_index -= 1;
                        } else {
                            state.menu_state.selected_index = state.menu_state.options.len() - 1;
                        }
                    }
                    MenuSubState::SaveSelect => {
                        if state.menu_state.selected_index > 0 {
                            state.menu_state.selected_index -= 1;
                        } else {
                            // Users + Back
                            let max_idx = state.system.users.len();
                            state.menu_state.selected_index = max_idx;
                        }
                    }
                    _ => {}
                }
            }
        }
        Key::Down => {
            if state.scene == Scene::Menu {
                match state.menu_state.sub_state {
                    MenuSubState::Main => {
                        if state.menu_state.selected_index < state.menu_state.options.len() - 1 {
                            state.menu_state.selected_index += 1;
                        } else {
                            state.menu_state.selected_index = 0;
                        }
                    }
                    MenuSubState::SaveSelect => {
                        let max_idx = state.system.users.len();
                        if state.menu_state.selected_index < max_idx {
                            state.menu_state.selected_index += 1;
                        } else {
                            state.menu_state.selected_index = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn handle_escape_key(state: &mut GameState) {
    match state.scene {
        Scene::Desktop => {
            state.scene = Scene::Menu;
            state.menu_state.sub_state = MenuSubState::Main;
        }
        Scene::Menu => {
            match state.menu_state.sub_state {
                MenuSubState::Main => {
                    if state.session_started {
                        state.scene = Scene::Desktop;
                    }
                }
                MenuSubState::SaveSelect | MenuSubState::Settings | MenuSubState::Credits => {
                    state.menu_state.sub_state = MenuSubState::Main;
                    state.menu_state.selected_index = 0;
                }
                MenuSubState::CreateSave => {
                    state.menu_state.sub_state = MenuSubState::SaveSelect;
                    state.menu_state.input_buffer.clear();
                    state.menu_state.error_message = None;
                }
            }
        }
        _ => {}
    }
}

fn handle_enter_key(_ctx: &mut Context, state: &mut GameState) {
    match state.scene {
        Scene::Menu => {
            match state.menu_state.sub_state {
                MenuSubState::Main => {
                    match state.menu_state.selected_index {
                        0 => { // Start Game
                            if state.system.users.is_empty() {
                                state.menu_state.sub_state = MenuSubState::CreateSave;
                                state.menu_state.input_buffer.clear();
                            } else {
                                // Use top user
                                state.system.current_user = Some(state.system.users[0].clone());
                                start_game(state);
                            }
                        }
                        1 => { // Create Save
                            state.menu_state.sub_state = MenuSubState::CreateSave;
                            state.menu_state.input_buffer.clear();
                        }
                        2 => { // Select Save
                            state.menu_state.sub_state = MenuSubState::SaveSelect;
                            state.menu_state.selected_index = 0;
                        }
                        3 => { // Settings
                            state.menu_state.sub_state = MenuSubState::Settings;
                        }
                        4 => { // Credits
                            state.menu_state.sub_state = MenuSubState::Credits;
                        }
                        5 => { // Exit
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                MenuSubState::SaveSelect => {
                    let users_len = state.system.users.len();
                    if state.menu_state.selected_index < users_len {
                        // Select existing user and move to top
                        state.system.set_user_as_top(state.menu_state.selected_index);
                        state.system.current_user = Some(state.system.users[0].clone());
                        state.menu_state.sub_state = MenuSubState::Main;
                        state.menu_state.selected_index = 0;
                    } else {
                        // Back
                        state.menu_state.sub_state = MenuSubState::Main;
                        state.menu_state.selected_index = 0;
                    }
                }
                MenuSubState::CreateSave => {
                    let name = state.menu_state.input_buffer.trim().to_string();
                    if name.is_empty() {
                        state.menu_state.error_message = Some("Name cannot be empty".to_string());
                    } else if state.system.users.iter().any(|u| u.username == name) {
                        state.menu_state.error_message = Some("Name already exists".to_string());
                    } else {
                        let new_user = User {
                            username: name,
                            teblig_count: 0,
                            cihad_count: 0,
                            tekfir_count: 0,
                            current_stage: 1,
                        };
                        state.system.users.insert(0, new_user); // Insert at top
                        state.system.save_users();
                        state.system.current_user = Some(state.system.users[0].clone());
                        
                        // Go back to main menu or start game? User said "menüye girerken save açılacak... eğer varsa en tepedekini kullanacak"
                        // Let's go back to main menu so they can click Start
                        state.menu_state.sub_state = MenuSubState::Main;
                        state.menu_state.selected_index = 0;
                    }
                }
                _ => {}
            }
        }
        Scene::KernelPanic => {
            state.reset();
        }
        _ => {}
    }
}

fn start_game(state: &mut GameState) {
    state.scene = Scene::TransitionToDesktop;
    state.transition_timer = 0.0;
    state.session_started = true;
    // Reset game state on start
    state.player.health = 100.0;
    
    if let Some(user) = &state.system.current_user {
        state.world.current_stage = user.current_stage as u8;
    } else {
        state.world.current_stage = 1;
    }

    state.player.pos = Vec2::new(400.0, 300.0);
    state.player.direction = Direction::Front;
}

