use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Rectangle};
use tetra::input::{self, Key, MouseButton};
use tetra::Event;
use tetra::math::{Vec2, Vec3, Mat4};
use tetra::{Context, ContextBuilder, State};
use rand::Rng;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

// --- Game Structs ---
#[derive(Clone)]
struct Icon {
    rect: Rectangle,
    label: String,
    action: IconAction,
    color: Color,
}

#[derive(Clone, Copy)]
enum IconAction {
    OpenMail,
    StartGame,
    OpenTerminal,
    OpenChat,
}

#[derive(PartialEq, Clone, Copy)]
enum WindowKind {
    Info,
    Terminal,
    Chat,
}

struct Window {
    rect: Rectangle,
    title: String,
    content: String,
    visible: bool,
    kind: WindowKind,
}

struct Bullet {
    pos: Vec2<f32>,
    vel: Vec2<f32>,
}

struct Enemy {
    pos: Vec2<f32>,
    size: f32,
}

struct ChatMessage {
    sender: String,
    content: String,
    color: Color,
}

#[derive(PartialEq)]
enum NpcState {
    Intro,
    WaitingForHash,
    Success,
}

struct ChatSystem {
    messages: Vec<ChatMessage>,
    input_buffer: String,
    npc_state: NpcState,
    response_timer: f32,
}

impl ChatSystem {
    fn new() -> Self {
        ChatSystem {
            messages: vec![
                ChatMessage { 
                    sender: "System".to_string(), 
                    content: "Connecting to #agalar-dev...".to_string(), 
                    color: Color::rgb(0.5, 0.5, 0.5) 
                },
                ChatMessage { 
                    sender: "System".to_string(), 
                    content: "Connected.".to_string(), 
                    color: Color::rgb(0.5, 0.5, 0.5) 
                },
            ],
            input_buffer: String::new(),
            npc_state: NpcState::Intro,
            response_timer: 2.0, // Initial delay for NPC to speak
        }
    }
}

struct MiniGame {
    active: bool,
    player_pos: Vec2<f32>,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    score: u32,
    timer: f32,
    spawn_timer: f32,
    game_over: bool,
    level: u32,
}

impl MiniGame {
    fn new() -> Self {
        MiniGame {
            active: false,
            player_pos: Vec2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            timer: 0.0,
            spawn_timer: 0.0,
            game_over: false,
            level: 1,
        }
    }
}

enum Scene {
    Boot,
    LoginUsername,
    LoginPassword,
    Menu,
    TransitionToDesktop,
    Desktop,
    Config,
}

struct GameState {
    scene: Scene,
    font: Font,
    
    // Boot state
    boot_lines: Vec<String>,
    current_line: usize,
    current_char: usize,
    char_timer: f32,
    boot_complete_timer: f32,
    
    // Transition
    transition_timer: f32,
    
    // Login/Menu state
    input_buffer: String,
    login_error: Option<String>,
    
    menu_options: Vec<String>,
    selected_option: usize,
    cursor_timer: f32,
    cursor_visible: bool,
    
    // Timer to prevent immediate skipping of boot sequence
    boot_grace_timer: f32,

    // GUI Elements
    window_mesh: Mesh,
    title_bar_mesh: Mesh,
    cursor_mesh: Mesh,
    config_box_mesh: Mesh,
    config_shadow_mesh: Mesh,
    
    // Desktop State
    icons: Vec<Icon>,
    windows: Vec<Window>,
    mini_game: MiniGame,
    mail_read: bool,
    
    // New Systems
    chat_system: ChatSystem,
    terminal_input: String,
    terminal_history: Vec<String>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // Try to load a font. 
        let font_paths = [
            "resources/font.ttf",
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "/usr/share/fonts/truetype/freefont/FreeMono.ttf",
            "/usr/share/fonts/liberation/LiberationMono-Regular.ttf",
            "C:\\Windows\\Fonts\\consola.ttf", // Just in case
        ];

        let mut font = None;
        for path in &font_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(f) = Font::vector(ctx, path, 16.0) {
                    font = Some(f);
                    break;
                }
            }
        }

        let font = match font {
            Some(f) => f,
            None => panic!("Could not find a suitable font! Please place 'font.ttf' in the 'resources' folder."),
        };

        // Initialize Meshes
        let window_mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, 400.0, 300.0),
        )?;
        
        let title_bar_mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, 400.0, 30.0),
        )?;

        let cursor_mesh = Mesh::polygon(
            ctx,
            ShapeStyle::Fill,
            &[
                Vec2::new(0.0, 0.0),
                Vec2::new(0.0, 20.0),
                Vec2::new(15.0, 15.0),
            ],
        )?;

        let config_box_mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, 600.0, 400.0),
        )?;

        let config_shadow_mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, 600.0, 400.0),
        )?;

        // Setup Desktop Icons
        let icons = vec![
            Icon {
                rect: Rectangle::new(20.0, 20.0, 64.0, 64.0),
                label: "Mail".to_string(),
                action: IconAction::OpenMail,
                color: Color::rgb(0.9, 0.9, 0.0), // Yellow
            },
            Icon {
                rect: Rectangle::new(20.0, 120.0, 64.0, 64.0),
                label: "System_Def".to_string(),
                action: IconAction::StartGame,
                color: Color::rgb(0.8, 0.2, 0.2), // Red
            },
            Icon {
                rect: Rectangle::new(20.0, 220.0, 64.0, 64.0),
                label: "Terminal".to_string(),
                action: IconAction::OpenTerminal,
                color: Color::rgb(0.2, 0.2, 0.2), // Black
            },
            Icon {
                rect: Rectangle::new(20.0, 320.0, 64.0, 64.0),
                label: "Chat".to_string(),
                action: IconAction::OpenChat,
                color: Color::rgb(0.0, 0.5, 0.0), // Green
            },
        ];

        Ok(GameState {
            scene: Scene::Boot,
            font,
            boot_lines: vec![
                "Starting version 245.4-1-arch...".to_string(),
                "/dev/sda1: clean, 245/640 files, 123/245 blocks".to_string(),
                "[  OK  ] Started Dispatch Password Requests to Console Directory Watch.".to_string(),
                "[  OK  ] Reached target Local Encrypted Volumes.".to_string(),
                "[  OK  ] Reached target Paths.".to_string(),
                "[  OK  ] Reached target Remote File Systems.".to_string(),
                "[  OK  ] Reached target Slice Units.".to_string(),
                "[  OK  ] Reached target Swap.".to_string(),
                "[  OK  ] Listening on Device-mapper event daemon FIFOs.".to_string(),
                "[  OK  ] Listening on LVM2 poll daemon socket.".to_string(),
                "[  OK  ] Listening on Process Core Dump Socket.".to_string(),
                "[  OK  ] Listening on Journal Socket.".to_string(),
                "[  OK  ] Started Remount Root and Kernel File Systems.".to_string(),
                "[  OK  ] Started Create System Users.".to_string(),
                "[  OK  ] Started Journal Service.".to_string(),
                "         Starting Flush Journal to Persistent Storage...".to_string(),
                "[  OK  ] Finished Flush Journal to Persistent Storage.".to_string(),
                "[  OK  ] Started Network Service.".to_string(),
                "[  OK  ] Reached target Network.".to_string(),
                "         Starting Network Name Resolution...".to_string(),
                "[  OK  ] Started Network Name Resolution.".to_string(),
                "[  OK  ] Reached target Host and Network Name Lookups.".to_string(),
                "[  OK  ] Started User Login Management.".to_string(),
                "Welcome to Agalar Linux 1.0 LTS (tty1)".to_string(),
                " ".to_string(),
            ],
            current_line: 0,
            current_char: 0,
            char_timer: 0.0,
            boot_complete_timer: 0.0,
            transition_timer: 0.0,
            
            menu_options: vec![
                "1. Start X Server".to_string(),
                "2. System Config".to_string(),
                "3. Logout".to_string(),
                "4. Reboot".to_string(),
                "5. Shutdown".to_string(),
            ],
            selected_option: 0,
            input_buffer: String::new(),
            login_error: None,
            cursor_timer: 0.0,
            cursor_visible: true,
            boot_grace_timer: 0.0,
            
            window_mesh,
            title_bar_mesh,
            cursor_mesh,
            config_box_mesh,
            config_shadow_mesh,
            
            icons,
            windows: Vec::new(),
            mini_game: MiniGame::new(),
            mail_read: false,
            chat_system: ChatSystem::new(),
            terminal_input: String::new(),
            terminal_history: Vec::new(),
        })
    }

    fn reset(&mut self) {
        self.scene = Scene::Boot;
        self.current_line = 0;
        self.current_char = 0;
        self.char_timer = 0.0;
        self.boot_complete_timer = 0.0;
        self.boot_grace_timer = 0.0;
        self.input_buffer.clear();
        self.login_error = None;
        self.selected_option = 0;
    }

    fn logout(&mut self) {
        self.scene = Scene::LoginUsername;
        self.input_buffer.clear();
        self.login_error = None;
        self.selected_option = 0;
        self.windows.clear();
        self.mini_game = MiniGame::new();
    }

    fn open_window(&mut self, title: &str, content: &str, kind: WindowKind) {
        self.windows.push(Window {
            rect: Rectangle::new(200.0, 150.0, 400.0, 300.0),
            title: title.to_string(),
            content: content.to_string(),
            visible: true,
            kind,
        });
    }
}

impl State for GameState {
    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        match event {
            Event::MouseButtonPressed { button: MouseButton::Left } => {
                if let Scene::Desktop = self.scene {
                    let mouse_pos = input::get_mouse_position(ctx);
                    
                    if self.mini_game.active {
                        // Shooting in game
                        let dir = (mouse_pos - self.mini_game.player_pos).normalized();
                        self.mini_game.bullets.push(Bullet {
                            pos: self.mini_game.player_pos,
                            vel: dir * 10.0,
                        });
                    } else {
                        // Desktop interaction
                        // Check windows (top to bottom)
                        let mut window_clicked = false;
                        // Iterate in reverse to click top window first
                        for i in (0..self.windows.len()).rev() {
                            let win = &self.windows[i];
                            if win.visible && win.rect.contains_point(mouse_pos) {
                                // Close button logic (top right corner)
                                let close_btn = Rectangle::new(win.rect.x + win.rect.width - 30.0, win.rect.y, 30.0, 30.0);
                                if close_btn.contains_point(mouse_pos) {
                                    self.windows.remove(i);
                                }
                                window_clicked = true;
                                break;
                            }
                        }

                        if !window_clicked {
                            // Check icons
                            let mut clicked_action = None;
                            for icon in &self.icons {
                                if icon.rect.contains_point(mouse_pos) {
                                    clicked_action = Some(icon.action);
                                    break;
                                }
                            }

                            if let Some(action) = clicked_action {
                                match action {
                                    IconAction::OpenMail => {
                                        self.open_window("Inbox - 1 Unread", "FROM: Unknown\nSUBJECT: SYSTEM CORRUPTION\n\nThe system is under attack. Unknown entities are\ncorrupting the memory blocks.\n\nI have installed a defense protocol 'System_Def'.\nRun it to purge the corruption.\n\nContact 'Glitch' on IRC channel #agalar-dev for more info.", WindowKind::Info);
                                        self.mail_read = true;
                                    }
                                    IconAction::StartGame => {
                                        self.mini_game.active = true;
                                        self.mini_game.game_over = false;
                                        self.mini_game.score = 0;
                                        self.mini_game.timer = 60.0; // Survive 60 seconds
                                        self.mini_game.enemies.clear();
                                        self.mini_game.bullets.clear();
                                    }
                                    IconAction::OpenTerminal => {
                                        self.open_window("Terminal", "root@agalar:~# ", WindowKind::Terminal);
                                        self.terminal_input.clear();
                                    }
                                    IconAction::OpenChat => {
                                        self.open_window("IRC - #agalar-dev", "", WindowKind::Chat);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Event::TextInput { text } => {
                match self.scene {
                    Scene::LoginUsername | Scene::LoginPassword => {
                        // Filter out control characters if any slip through
                        if !text.chars().any(|c: char| c.is_control()) {
                            self.input_buffer.push_str(&text);
                        }
                    }
                    Scene::Desktop => {
                        // Check active window
                        if let Some(win) = self.windows.last_mut() {
                            if win.visible {
                                match win.kind {
                                    WindowKind::Chat => {
                                        if !text.chars().any(|c: char| c.is_control()) {
                                            self.chat_system.input_buffer.push_str(&text);
                                        }
                                    }
                                    WindowKind::Terminal => {
                                        if !text.chars().any(|c: char| c.is_control()) {
                                            self.terminal_input.push_str(&text);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::KeyPressed { key: Key::Backspace } => {
                match self.scene {
                    Scene::LoginUsername | Scene::LoginPassword => {
                        self.input_buffer.pop();
                    }
                    Scene::Desktop => {
                        if let Some(win) = self.windows.last_mut() {
                            if win.visible {
                                match win.kind {
                                    WindowKind::Chat => {
                                        self.chat_system.input_buffer.pop();
                                    }
                                    WindowKind::Terminal => {
                                        self.terminal_input.pop();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::KeyPressed { key: Key::Enter } => {
                match self.scene {
                    Scene::Desktop => {
                        if let Some(win) = self.windows.last_mut() {
                            if win.visible {
                                match win.kind {
                                    WindowKind::Chat => {
                                        let msg = self.chat_system.input_buffer.clone();
                                        if !msg.is_empty() {
                                            self.chat_system.messages.push(ChatMessage {
                                                sender: "root".to_string(),
                                                content: msg.clone(),
                                                color: Color::WHITE,
                                            });
                                            self.chat_system.input_buffer.clear();
                                            
                                            // Simple NPC Logic Trigger
                                            if self.chat_system.npc_state == NpcState::WaitingForHash {
                                                if msg.trim() == "0xDEADBEEF" {
                                                    self.chat_system.npc_state = NpcState::Success;
                                                    self.chat_system.response_timer = 1.0;
                                                } else {
                                                    self.chat_system.messages.push(ChatMessage {
                                                        sender: "Glitch".to_string(),
                                                        content: "Invalid hash. Try again.".to_string(),
                                                        color: Color::RED,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                    WindowKind::Terminal => {
                                        let cmd = self.terminal_input.clone();
                                        self.terminal_history.push(format!("root@agalar:~# {}", cmd));
                                        
                                        if cmd.trim() == "sys_check" {
                                            self.terminal_history.push("Scanning system integrity...".to_string());
                                            self.terminal_history.push("[WARN] Kernel corruption detected.".to_string());
                                            self.terminal_history.push("Integrity Hash: 0xDEADBEEF".to_string());
                                        } else if cmd.trim() == "clear" {
                                            self.terminal_history.clear();
                                        } else if !cmd.trim().is_empty() {
                                            self.terminal_history.push(format!("bash: {}: command not found", cmd));
                                        }
                                        
                                        self.terminal_input.clear();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Scene::LoginUsername => {
                        if self.input_buffer == "root" {
                            self.scene = Scene::LoginPassword;
                            self.input_buffer.clear();
                            self.login_error = None;
                        } else {
                            self.login_error = Some("Login incorrect".to_string());
                            self.input_buffer.clear();
                            // Reset to username after a short delay or immediately? 
                            // For simplicity, just clear and stay on username
                        }
                    }
                    Scene::LoginPassword => {
                        // Accept any password
                        self.scene = Scene::Menu;
                        self.input_buffer.clear();
                    }
                    Scene::Menu => {
                        match self.selected_option {
                            0 => {
                                self.scene = Scene::TransitionToDesktop;
                                self.transition_timer = 0.0;
                            }
                            1 => self.scene = Scene::Config,
                            2 => self.logout(),
                            3 => self.reset(),
                            4 => std::process::exit(0),
                            _ => {}
                        }
                    }
                    Scene::Config => {
                        // Exit config on Enter for now
                        self.scene = Scene::Menu;
                    }
                    _ => {}
                }
            }
            Event::KeyPressed { key: Key::Escape } => {
                match self.scene {
                    Scene::Desktop | Scene::Config => {
                        self.scene = Scene::Menu;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scene {
            Scene::Boot => {
                // Typing effect
                self.char_timer += 1.0; 
                self.boot_grace_timer += 1.0;

                if self.char_timer > 0.5 { // Faster typing for systemd feel
                    self.char_timer = 0.0;
                    if self.current_line < self.boot_lines.len() {
                        let line = &self.boot_lines[self.current_line];
                        if self.current_char < line.len() {
                            // Type multiple chars at once for speed
                            self.current_char += 3;
                            if self.current_char > line.len() {
                                self.current_char = line.len();
                            }
                        } else {
                            self.current_line += 1;
                            self.current_char = 0;
                            self.char_timer = -5.0; // Short pause between lines
                        }
                    } else {
                        self.boot_complete_timer += 1.0;
                        if self.boot_complete_timer > 60.0 {
                            self.scene = Scene::LoginUsername;
                        }
                    }
                }
                
                // Skip boot with Enter, but only after grace period
                if self.boot_grace_timer > 30.0 && input::is_key_pressed(ctx, Key::Enter) {
                    self.scene = Scene::LoginUsername;
                }
            }
            Scene::LoginUsername | Scene::LoginPassword => {
                // Cursor blinking
                self.cursor_timer += 1.0;
                if self.cursor_timer > 30.0 {
                    self.cursor_timer = 0.0;
                    self.cursor_visible = !self.cursor_visible;
                }
            }
            Scene::Menu => {
                // Input handling
                if input::is_key_pressed(ctx, Key::Up) {
                    if self.selected_option > 0 {
                        self.selected_option -= 1;
                    }
                }
                if input::is_key_pressed(ctx, Key::Down) {
                    if self.selected_option < self.menu_options.len() - 1 {
                        self.selected_option += 1;
                    }
                }
            }
            Scene::TransitionToDesktop => {
                self.transition_timer += 1.0;
                if self.transition_timer > 120.0 { // 2 seconds fade
                    self.scene = Scene::Desktop;
                }
            }
            Scene::Desktop => {
                // NPC Chat Logic
                if self.chat_system.response_timer > 0.0 {
                    self.chat_system.response_timer -= 1.0 / 60.0;
                    if self.chat_system.response_timer <= 0.0 {
                        match self.chat_system.npc_state {
                            NpcState::Intro => {
                                self.chat_system.messages.push(ChatMessage {
                                    sender: "Glitch".to_string(),
                                    content: "Who is this? Are you the admin?".to_string(),
                                    color: Color::rgb(1.0, 1.0, 0.0),
                                });
                                self.chat_system.npc_state = NpcState::WaitingForHash;
                                // Actually let's make it simpler, just ask for hash immediately after intro
                                self.chat_system.messages.push(ChatMessage {
                                    sender: "Glitch".to_string(),
                                    content: "If you are, prove it. Give me the system integrity hash.".to_string(),
                                    color: Color::rgb(1.0, 1.0, 0.0),
                                });
                                self.chat_system.messages.push(ChatMessage {
                                    sender: "Glitch".to_string(),
                                    content: "Run 'sys_check' in the terminal.".to_string(),
                                    color: Color::rgb(1.0, 1.0, 0.0),
                                });
                            }
                            NpcState::Success => {
                                self.chat_system.messages.push(ChatMessage {
                                    sender: "Glitch".to_string(),
                                    content: "Hash verified. You are the admin.".to_string(),
                                    color: Color::GREEN,
                                });
                                self.chat_system.messages.push(ChatMessage {
                                    sender: "Glitch".to_string(),
                                    content: "I've unlocked the 'Deep Scan' mode in System_Def.".to_string(),
                                    color: Color::GREEN,
                                });
                                self.chat_system.messages.push(ChatMessage {
                                    sender: "Glitch".to_string(),
                                    content: "Go purge them all.".to_string(),
                                    color: Color::GREEN,
                                });
                                // Unlock something in game (e.g. level 2 or just stronger weapon)
                                self.mini_game.level = 2; 
                            }
                            _ => {}
                        }
                    }
                }

                if self.mini_game.active {
                    if !self.mini_game.game_over {
                        // Player Movement
                        let speed = 5.0;
                        if input::is_key_down(ctx, Key::W) { self.mini_game.player_pos.y -= speed; }
                        if input::is_key_down(ctx, Key::S) { self.mini_game.player_pos.y += speed; }
                        if input::is_key_down(ctx, Key::A) { self.mini_game.player_pos.x -= speed; }
                        if input::is_key_down(ctx, Key::D) { self.mini_game.player_pos.x += speed; }

                        // Clamp player to screen
                        self.mini_game.player_pos.x = self.mini_game.player_pos.x.clamp(0.0, SCREEN_WIDTH as f32);
                        self.mini_game.player_pos.y = self.mini_game.player_pos.y.clamp(0.0, SCREEN_HEIGHT as f32);

                        // Update Bullets
                        self.mini_game.bullets.retain_mut(|b| {
                            b.pos += b.vel;
                            b.pos.x > 0.0 && b.pos.x < SCREEN_WIDTH as f32 && b.pos.y > 0.0 && b.pos.y < SCREEN_HEIGHT as f32
                        });

                        // Spawn Enemies
                        self.mini_game.spawn_timer += 1.0;
                        // Increase difficulty based on level
                        let spawn_rate = if self.mini_game.level > 1 { 30.0 } else { 60.0 };
                        
                        if self.mini_game.spawn_timer > spawn_rate { 
                            self.mini_game.spawn_timer = 0.0;
                            let mut rng = rand::thread_rng();
                            let side = rng.gen_range(0..4);
                            let pos = match side {
                                0 => Vec2::new(rng.gen_range(0.0..SCREEN_WIDTH as f32), -20.0), // Top
                                1 => Vec2::new(rng.gen_range(0.0..SCREEN_WIDTH as f32), SCREEN_HEIGHT as f32 + 20.0), // Bottom
                                2 => Vec2::new(-20.0, rng.gen_range(0.0..SCREEN_HEIGHT as f32)), // Left
                                _ => Vec2::new(SCREEN_WIDTH as f32 + 20.0, rng.gen_range(0.0..SCREEN_HEIGHT as f32)), // Right
                            };
                            self.mini_game.enemies.push(Enemy { pos, size: 20.0 });
                        }

                        // Update Enemies
                        let player_pos = self.mini_game.player_pos;
                        let enemy_speed = if self.mini_game.level > 1 { 3.0 } else { 2.0 };
                        for enemy in &mut self.mini_game.enemies {
                            let dir = (player_pos - enemy.pos).normalized();
                            enemy.pos += dir * enemy_speed;
                        }

                        // Collision: Bullet vs Enemy
                        let mut bullets_to_remove = Vec::new();
                        let mut enemies_to_remove = Vec::new();

                        for (b_idx, bullet) in self.mini_game.bullets.iter().enumerate() {
                            for (e_idx, enemy) in self.mini_game.enemies.iter().enumerate() {
                                if bullet.pos.distance(enemy.pos) < enemy.size {
                                    bullets_to_remove.push(b_idx);
                                    enemies_to_remove.push(e_idx);
                                    self.mini_game.score += 10;
                                }
                            }
                        }
                        
                        // Remove collided (simple approach, might miss some if multiple hit same frame but ok for simple game)
                        // Sort and dedup to avoid index issues
                        bullets_to_remove.sort(); bullets_to_remove.dedup();
                        enemies_to_remove.sort(); enemies_to_remove.dedup();
                        
                        for i in bullets_to_remove.iter().rev() { self.mini_game.bullets.remove(*i); }
                        for i in enemies_to_remove.iter().rev() { self.mini_game.enemies.remove(*i); }

                        // Collision: Enemy vs Player
                        for enemy in &self.mini_game.enemies {
                            if enemy.pos.distance(player_pos) < 20.0 {
                                self.mini_game.game_over = true;
                            }
                        }

                        // Timer
                        self.mini_game.timer -= 1.0 / 60.0; // Approx dt
                        if self.mini_game.timer <= 0.0 {
                            self.mini_game.game_over = true; // Win condition actually, but stop game loop
                            // Handle Win
                            self.open_window("System Alert", "THREAT ELIMINATED.\n\nCore integrity restored.\nNew data decrypted: 'The Architect lives.'", WindowKind::Info);
                            self.mini_game.active = false;
                        }
                    } else {
                        if input::is_key_pressed(ctx, Key::R) {
                            // Restart
                            self.mini_game.active = true;
                            self.mini_game.game_over = false;
                            self.mini_game.score = 0;
                            self.mini_game.timer = 60.0;
                            self.mini_game.enemies.clear();
                            self.mini_game.bullets.clear();
                        }
                        if input::is_key_pressed(ctx, Key::Escape) {
                            self.mini_game.active = false;
                        }
                    }
                } else {
                    // Simple desktop logic (maybe move cursor with mouse later)
                }
            }
            Scene::Config => {
                // Config logic
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        match self.scene {
            Scene::Boot => {
                let mut y = 20.0;
                // Only draw last 25 lines to simulate scrolling if needed, but for now just draw all
                // Or better, draw from current_line - 25
                let start_line = if self.current_line > 25 { self.current_line - 25 } else { 0 };
                
                for (i, line) in self.boot_lines.iter().enumerate().skip(start_line) {
                    if i < self.current_line {
                        // Check for [ OK ]
                        if line.starts_with("[  OK  ]") {
                            let mut ok_part = Text::new("[  OK  ]", self.font.clone());
                            ok_part.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::GREEN));
                            
                            let rest = &line[8..];
                            let mut rest_text = Text::new(rest, self.font.clone());
                            let ok_width = ok_part.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                            rest_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + ok_width, y)).color(Color::WHITE));
                        } else {
                            let mut text = Text::new(line, self.font.clone());
                            text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                        }
                    } else if i == self.current_line {
                        let sub = &line[0..self.current_char];
                        // Simple drawing for current line, no color parsing for partial line to keep it simple
                        let mut text = Text::new(sub, self.font.clone());
                        // Actually if it starts with [ OK ] we want the OK green and rest white, but for typing effect simple is okay
                        // Let's just make it white while typing
                        text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                        
                        // Draw cursor
                        if (self.char_timer as i32 / 5) % 2 == 0 {
                             let mut cursor = Text::new("_", self.font.clone());
                             let width = text.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                             cursor.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + width, y)).color(Color::WHITE));
                        }
                    }
                    y += 20.0;
                }
            }
            Scene::LoginUsername | Scene::LoginPassword => {
                let mut y = 20.0;
                
                // Draw "agalar login: "
                let login_prompt = "agalar login: ";
                let mut prompt_text = Text::new(login_prompt, self.font.clone());
                prompt_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                
                if let Scene::LoginUsername = self.scene {
                    let input_display = format!("{}{}", self.input_buffer, if self.cursor_visible { "_" } else { "" });
                    let mut input_text = Text::new(input_display, self.font.clone());
                    let prompt_width = prompt_text.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                    input_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + prompt_width, y)).color(Color::WHITE));
                } else {
                    // If password state, draw "root" as already entered
                    let mut root_text = Text::new("root", self.font.clone());
                    let prompt_width = prompt_text.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                    root_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + prompt_width, y)).color(Color::WHITE));
                    
                    y += 24.0;
                    let pass_prompt = "Password: ";
                    let mut pass_text = Text::new(pass_prompt, self.font.clone());
                    pass_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                    
                    let masked_input: String = self.input_buffer.chars().map(|_| '*').collect();
                    let input_display = format!("{}{}", masked_input, if self.cursor_visible { "_" } else { "" });
                    let mut input_text = Text::new(input_display, self.font.clone());
                    let pass_width = pass_text.get_bounds(ctx).map(|b| b.width).unwrap_or(0.0);
                    input_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0 + pass_width, y)).color(Color::WHITE));
                }

                if let Some(err) = &self.login_error {
                    y += 24.0;
                    let mut err_text = Text::new(err, self.font.clone());
                    err_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::RED));
                }
            }
            Scene::Menu | Scene::TransitionToDesktop => {
                let t = if let Scene::TransitionToDesktop = self.scene {
                    (self.transition_timer / 120.0).clamp(0.0, 1.0)
                } else {
                    0.0
                };

                if t > 0.0 {
                    let scale = 1.0 + t * t * 2.0; // Strong zoom
                    // Center is 400, 300
                    let trans_to_origin = Mat4::<f32>::translation_3d(Vec3::new(-400.0, -300.0, 0.0));
                    let scaling = Mat4::<f32>::scaling_3d(Vec3::new(scale, scale, 1.0));
                    let trans_back = Mat4::<f32>::translation_3d(Vec3::new(400.0, 300.0, 0.0));
                    
                    let transform = trans_back * scaling * trans_to_origin;
                    graphics::set_transform_matrix(ctx, transform);
                }

                // Draw Login Prompt (Static history)
                let mut y = 20.0;
                let mut login_text = Text::new("agalar login: root", self.font.clone());
                login_text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                
                y += 24.0;
                
                y += 24.0;
                let mut welcome = Text::new("Last login: Fri Dec 27 12:00:00 on tty1", self.font.clone());
                welcome.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                y += 40.0;

                // Draw Menu Options as a shell script output or similar
                for (i, option) in self.menu_options.iter().enumerate() {
                    let prefix = if i == self.selected_option { "> " } else { "  " };
                    let full_text = format!("{}{}", prefix, option);
                    
                    let color = if i == self.selected_option {
                        Color::GREEN
                    } else {
                        Color::WHITE
                    };
                    
                    let mut text = Text::new(full_text, self.font.clone());
                    text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(color));
                    y += 20.0;
                }
                
                // Transition Effect
                if t > 0.0 {
                    graphics::set_transform_matrix(ctx, Mat4::identity());
                    
                    let alpha = t * t * t; // Cubic ease-in
                    let fade_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
                    fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(1.0, 1.0, 1.0, alpha)));
                }
            }
            Scene::Desktop => {
                if self.mini_game.active {
                    // Draw Game
                    graphics::clear(ctx, Color::rgb(0.1, 0.0, 0.0)); // Dark Red background
                    
                    // Draw Grid
                    // ... (skip for brevity, just simple background)

                    // Draw Player
                    let player_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 20.0, 20.0)).unwrap();
                    player_rect.draw(ctx, DrawParams::new().position(self.mini_game.player_pos - Vec2::new(10.0, 10.0)).color(Color::GREEN));

                    // Draw Enemies
                    let enemy_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 20.0, 20.0)).unwrap();
                    for enemy in &self.mini_game.enemies {
                        enemy_rect.draw(ctx, DrawParams::new().position(enemy.pos - Vec2::new(10.0, 10.0)).color(Color::RED));
                    }

                    // Draw Bullets
                    let bullet_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 5.0, 5.0)).unwrap();
                    for bullet in &self.mini_game.bullets {
                        bullet_rect.draw(ctx, DrawParams::new().position(bullet.pos).color(Color::rgb(1.0, 1.0, 0.0)));
                    }

                    // UI
                    let mut score_text = Text::new(format!("Score: {}", self.mini_game.score), self.font.clone());
                    score_text.draw(ctx, DrawParams::new().position(Vec2::new(10.0, 10.0)).color(Color::WHITE));
                    
                    let mut time_text = Text::new(format!("Time: {:.1}", self.mini_game.timer), self.font.clone());
                    time_text.draw(ctx, DrawParams::new().position(Vec2::new(SCREEN_WIDTH as f32 - 150.0, 10.0)).color(Color::WHITE));

                    if self.mini_game.game_over && self.mini_game.timer > 0.0 {
                        let mut over_text = Text::new("GAME OVER - Press R to Restart, ESC to Exit", self.font.clone());
                        over_text.draw(ctx, DrawParams::new().position(Vec2::new(200.0, 300.0)).color(Color::RED));
                    }

                } else {
                    graphics::clear(ctx, Color::rgb(0.0, 0.5, 0.5)); // Teal background
                    
                    // Draw Icons
                    for icon in &self.icons {
                        let icon_mesh = Mesh::rectangle(ctx, ShapeStyle::Fill, icon.rect).unwrap();
                        icon_mesh.draw(ctx, DrawParams::new().color(icon.color));
                        
                        let mut label = Text::new(&icon.label, self.font.clone());
                        label.draw(ctx, DrawParams::new().position(Vec2::new(icon.rect.x, icon.rect.y + 70.0)).color(Color::WHITE));
                    }

                    // Draw Windows
                    for win in &self.windows {
                        if win.visible {
                            // Window Body
                            self.window_mesh.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x, win.rect.y)).color(Color::rgb(0.8, 0.8, 0.8)));
                            
                            // Title Bar
                            self.title_bar_mesh.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x, win.rect.y)).color(Color::rgb(0.0, 0.0, 0.5)));
                            
                            let mut title = Text::new(&win.title, self.font.clone());
                            title.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x + 10.0, win.rect.y + 5.0)).color(Color::WHITE));
                            
                            // Close Button (X)
                            let mut close = Text::new("X", self.font.clone());
                            close.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x + win.rect.width - 25.0, win.rect.y + 5.0)).color(Color::WHITE));

                            // Content
                            match win.kind {
                                WindowKind::Chat => {
                                    let mut y_offset = 40.0;
                                    // Draw messages (simple scrolling, just show last 10)
                                    let start_idx = if self.chat_system.messages.len() > 10 { self.chat_system.messages.len() - 10 } else { 0 };
                                    for msg in self.chat_system.messages.iter().skip(start_idx) {
                                        let display = format!("{}: {}", msg.sender, msg.content);
                                        let mut text = Text::new(display, self.font.clone());
                                        text.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x + 10.0, win.rect.y + y_offset)).color(msg.color));
                                        y_offset += 20.0;
                                    }
                                    
                                    // Draw Input Line
                                    let input_display = format!("> {}_", self.chat_system.input_buffer);
                                    let mut input_text = Text::new(input_display, self.font.clone());
                                    input_text.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x + 10.0, win.rect.y + 270.0)).color(Color::WHITE));
                                }
                                WindowKind::Terminal => {
                                    let mut y_offset = 40.0;
                                    // Draw history (last 10 lines)
                                    let start_idx = if self.terminal_history.len() > 10 { self.terminal_history.len() - 10 } else { 0 };
                                    for line in self.terminal_history.iter().skip(start_idx) {
                                        let mut text = Text::new(line, self.font.clone());
                                        text.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x + 10.0, win.rect.y + y_offset)).color(Color::WHITE));
                                        y_offset += 20.0;
                                    }
                                    
                                    // Draw Input Line
                                    let input_display = format!("root@agalar:~# {}_", self.terminal_input);
                                    let mut input_text = Text::new(input_display, self.font.clone());
                                    input_text.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x + 10.0, win.rect.y + y_offset)).color(Color::WHITE));
                                }
                                _ => {
                                    let mut content = Text::new(&win.content, self.font.clone());
                                    content.draw(ctx, DrawParams::new().position(Vec2::new(win.rect.x + 10.0, win.rect.y + 40.0)).color(Color::BLACK));
                                }
                            }
                        }
                    }
                    
                    // Mouse Cursor (Simple Triangle)
                    let mouse_pos = input::get_mouse_position(ctx);
                    self.cursor_mesh.draw(ctx, DrawParams::new().position(mouse_pos).color(Color::WHITE));
                }
            }
            Scene::Config => {
                graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.8)); // Blue background like BIOS/Dialog
                
                // Shadow
                self.config_shadow_mesh.draw(ctx, DrawParams::new().position(Vec2::new(110.0, 110.0)).color(Color::BLACK));
                
                // Draw a box
                self.config_box_mesh.draw(ctx, DrawParams::new().position(Vec2::new(100.0, 100.0)).color(Color::rgb(0.7, 0.7, 0.7)));

                let mut title = Text::new("System Configuration", self.font.clone());
                title.draw(ctx, DrawParams::new().position(Vec2::new(300.0, 120.0)).color(Color::BLACK));
                
                let mut content = Text::new(
                    "Hostname: agalar\nKernel: 5.10.0-agalar\nMemory: 245KB\n\n[ OK ] Save & Exit", 
                    self.font.clone()
                );
                content.draw(ctx, DrawParams::new().position(Vec2::new(150.0, 180.0)).color(Color::BLACK));
            }
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Agalar Linux Boot", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
