use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::Event;
use tetra::math::Vec2;
use tetra::{Context, State};

use crate::defs::{Scene, Direction, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::combat::CombatData;
use crate::player::PlayerState;
use crate::world::WorldState;
use crate::system::SystemState;

pub struct GameState {
    pub scene: Scene,
    pub font: Font,
    
    pub system: SystemState,
    pub player: PlayerState,
    pub world: WorldState,
    
    pub boot_state: crate::scenes::boot::BootState,
    pub menu_state: crate::scenes::menu::MenuState,
    
    // Transition
    pub transition_timer: f32,
    pub session_started: bool,
    
    // Timer to prevent immediate skipping of boot sequence
    pub boot_grace_timer: f32,

    // Combat
    pub combat_data: CombatData,
    pub heart_texture: Option<Texture>,
    pub bone_texture: Option<Texture>,
    pub fade_alpha: f32,
    pub fade_out: bool,
}



impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
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

        let mut boot_state = crate::scenes::boot::BootState::new();
        boot_state.boot_lines.push("Starting VibeCoded Linux version 6.9.420...".to_string());
        boot_state.boot_text_cache.push(None);

        Ok(GameState {
            scene: Scene::Boot,
            font,

            system: SystemState::new(ctx)?,
            player: PlayerState::new(),
            world: WorldState::new(),
            
            boot_state,
            menu_state: crate::scenes::menu::MenuState::new(),
            
            transition_timer: 0.0,
            session_started: false,
            boot_grace_timer: 0.0,
            
            combat_data: CombatData::new(),
            heart_texture: None,
            bone_texture: None,
            fade_alpha: 0.0,
            fade_out: false,
        })
    }

    pub fn generate_kernel_panic(&mut self) {
        self.system.generate_kernel_panic();
    }

    pub fn reset(&mut self) {
        self.scene = Scene::Boot;
        self.boot_state = crate::scenes::boot::BootState::new();
        self.boot_state.boot_lines.push("Starting VibeCoded Linux version 6.9.420...".to_string());
        self.boot_state.boot_text_cache.push(None);
        
        self.menu_state = crate::scenes::menu::MenuState::new();
        
        self.boot_grace_timer = 0.0;
        self.session_started = false;
        
        // Reset Game State
        self.player.health = 100.0;
        self.world.current_stage = 1;
        self.player.pos = Vec2::new(400.0, 300.0);
        self.player.direction = Direction::Front;
    }

    pub fn assign_asset(&mut self, index: usize, asset: crate::assets::LoadedAsset) {
        use crate::assets::LoadedAsset;
        match (index, asset) {
            (0, LoadedAsset::Texture(t)) => self.player.texture_front = Some(t),
            (1, LoadedAsset::Texture(t)) => self.player.texture_left = Some(t),
            (2, LoadedAsset::Texture(t)) => self.player.texture_right = Some(t),
            (3, LoadedAsset::Texture(t)) => self.world.bg_texture = Some(t),
            (4, LoadedAsset::Texture(t)) => self.world.npc_gaster_standing = Some(t),
            (5, LoadedAsset::Texture(t)) => self.world.npc_gaster_talking = Some(t),
            (6, LoadedAsset::Texture(t)) => self.world.rarity_texture = Some(t),
            (7, LoadedAsset::Texture(t)) => self.world.eilish_texture = Some(t),
            (8, LoadedAsset::Texture(t)) => self.world.sans_texture = Some(t),
            (9, LoadedAsset::Texture(t)) => self.world.sans_combat_texture = Some(t),
            (10, LoadedAsset::Texture(t)) => self.world.sans_shrug_texture = Some(t),
            (11, LoadedAsset::Texture(t)) => self.world.sans_handshake_texture = Some(t),
            (12, LoadedAsset::Texture(t)) => self.heart_texture = Some(t),
            (13, LoadedAsset::Texture(t)) => self.world.musicbox_texture = Some(t),
            (14, LoadedAsset::Sound(s)) => self.world.music_track = Some(s),
            (15, LoadedAsset::Texture(t)) => self.world.ayasofya_giris_texture = Some(t),
            (16, LoadedAsset::Texture(t)) => self.world.ayasofya_ici_texture = Some(t),
            (17, LoadedAsset::Texture(t)) => self.bone_texture = Some(t),
            (18, LoadedAsset::Texture(t)) => self.player.texture_fes = Some(t),
            (19, LoadedAsset::Texture(t)) => self.player.texture_takke = Some(t),
            _ => {
                println!("Warning: Asset index {} mismatch or unhandled", index);
            }
        }
    }

    pub fn is_asset_loaded(&self, index: usize) -> bool {
        match index {
            0 => self.player.texture_front.is_some(),
            1 => self.player.texture_left.is_some(),
            2 => self.player.texture_right.is_some(),
            3 => self.world.bg_texture.is_some(),
            4 => self.world.npc_gaster_standing.is_some(),
            5 => self.world.npc_gaster_talking.is_some(),
            6 => self.world.rarity_texture.is_some(),
            7 => self.world.eilish_texture.is_some(),
            8 => self.world.sans_texture.is_some(),
            9 => self.world.sans_combat_texture.is_some(),
            10 => self.world.sans_shrug_texture.is_some(),
            11 => self.world.sans_handshake_texture.is_some(),
            12 => self.heart_texture.is_some(),
            13 => self.world.musicbox_texture.is_some(),
            14 => self.world.music_track.is_some(),
            15 => self.world.ayasofya_giris_texture.is_some(),
            16 => self.world.ayasofya_ici_texture.is_some(),
            17 => self.bone_texture.is_some(),
            18 => self.player.texture_fes.is_some(),
            19 => self.player.texture_takke.is_some(),
            _ => false,
        }
    }
}

impl State for GameState {
    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        crate::input_handler::handle_event(ctx, self, event);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scene {
            Scene::Boot => {
                crate::scenes::boot::update(ctx, self)?;
            }
            Scene::Menu => {
                crate::scenes::menu::update(ctx, self)?;
            }
            Scene::TransitionToDesktop => {
                self.transition_timer += 1.0;
                if self.transition_timer > 120.0 { // 2 seconds fade
                    self.scene = Scene::Desktop;
                }
            }
            Scene::Desktop => {
                crate::scenes::desktop::update(ctx, self)?;
            }
            Scene::CombatTransition => {
                if self.fade_out {
                    self.fade_alpha += 0.02;
                    if self.fade_alpha >= 1.0 {
                        self.fade_alpha = 1.0;
                        self.scene = Scene::Combat;
                        self.fade_out = false;
                        // Reset combat data
                        self.combat_data = CombatData::new();
                    }
                }
            }
            Scene::Combat => {
                crate::scenes::combat::update(ctx, self)?;
            }
            Scene::KernelPanic => {
                if input::is_key_pressed(ctx, Key::Enter) {
                    self.reset();
                }
            }
            Scene::AyasofyaInside => {
                crate::scenes::ayasofya::update(ctx, self)?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        match self.scene {
            Scene::Boot => {
                crate::scenes::boot::draw(ctx, self)?;
            }
            Scene::Menu | Scene::TransitionToDesktop => {
                crate::scenes::menu::draw(ctx, self)?;
            }
            Scene::Desktop => {
                crate::scenes::desktop::draw(ctx, self)?;
            }
            Scene::CombatTransition => {
                // Draw Desktop underneath
                crate::scenes::desktop::draw(ctx, self)?;
                
                // Draw fade
                let fade_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
                fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, self.fade_alpha)));
            }
            Scene::Combat => {
                crate::scenes::combat::draw(ctx, self)?;
            }
            Scene::KernelPanic => {
                graphics::clear(ctx, Color::BLACK);
                
                let mut y = 20.0;
                for (i, line) in self.system.panic_report.iter().enumerate() {
                    let mut text = Text::new(line, self.font.clone());
                    
                    // Make the "Press ENTER" line blink
                    if i == self.system.panic_report.len() - 1 {
                        // Simple blink using frame count or similar (simulated with random for now or just static)
                        // Actually, let's just make it static for stability, or use a timer if we had one.
                        // We can use `ctx.get_time().as_secs_f32()` if we want.
                        // Let's just keep it white.
                        text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                    } else {
                        text.draw(ctx, DrawParams::new().position(Vec2::new(20.0, y)).color(Color::WHITE));
                    }
                    y += 20.0;
                }
            }
            Scene::AyasofyaInside => {
                crate::scenes::ayasofya::draw(ctx, self)?;
            }
        }

        Ok(())
    }
}
