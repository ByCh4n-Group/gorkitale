use tetra::graphics::{Color, Texture};
use tetra::audio::{Sound, SoundInstance};
use tetra::math::Vec2;

pub struct WorldState {
    pub current_stage: u8,
    pub bg_texture: Option<Texture>,
    
    // Gaster
    pub gaster_pos: Vec2<f32>,
    pub gaster_talking: bool,
    pub gaster_dialogues: Vec<String>,
    pub current_gaster_dialogue: String,
    pub npc_gaster_standing: Option<Texture>,
    pub npc_gaster_talking: Option<Texture>,

    // Rarity
    pub rarity_pos: Vec2<f32>,
    pub rarity_alive: bool,
    pub rarity_stabbed_timer: f32,
    pub rarity_texture: Option<Texture>,

    // Eilish
    pub eilish_pos: Vec2<f32>,
    pub eilish_talking: bool,
    pub eilish_dialogue_timer: f32,
    pub eilish_current_dialogue: String,
    pub eilish_texture: Option<Texture>,

    // MusicBox
    pub musicbox_pos: Vec2<f32>,
    pub music_playing: bool,
    pub disco_color: Color,
    pub disco_timer: f32,
    pub musicbox_texture: Option<Texture>,
    pub music_track: Option<Sound>,
    pub music_instance: Option<SoundInstance>,

    // Sans
    pub sans_pos: Vec2<f32>,
    pub sans_texture: Option<Texture>,
    pub sans_combat_texture: Option<Texture>,
    pub sans_shrug_texture: Option<Texture>,
    pub sans_handshake_texture: Option<Texture>,

    // Ayasofya
    pub ayasofya_giris_texture: Option<Texture>,
    pub ayasofya_ici_texture: Option<Texture>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            current_stage: 1,
            bg_texture: None,
            
            gaster_pos: Vec2::new(600.0, 300.0),
            gaster_talking: false,
            gaster_dialogues: vec![
                "çakar çakmaz çakan çakmak...".to_string(),
                "Beware the man who speaks in hands...".to_string(),
                "Dark, darker, yet darker...".to_string(),
                "The shadows cutting deeper...".to_string(),
                "Photon readings negative...".to_string(),
                "This next experiment seems very, very interesting...".to_string(),
                "What do you two think?".to_string(),
            ],
            current_gaster_dialogue: String::new(),
            npc_gaster_standing: None,
            npc_gaster_talking: None,

            rarity_pos: Vec2::new(150.0, 300.0),
            rarity_alive: true,
            rarity_stabbed_timer: 0.0,
            rarity_texture: None,

            eilish_pos: Vec2::new(150.0, 300.0),
            eilish_talking: false,
            eilish_dialogue_timer: 0.0,
            eilish_current_dialogue: String::new(),
            eilish_texture: None,

            musicbox_pos: Vec2::new(200.0, 300.0),
            music_playing: false,
            disco_color: Color::WHITE,
            disco_timer: 0.0,
            musicbox_texture: None,
            music_track: None,
            music_instance: None,

            sans_pos: Vec2::new(600.0, 300.0),
            sans_texture: None,
            sans_combat_texture: None,
            sans_shrug_texture: None,
            sans_handshake_texture: None,

            ayasofya_giris_texture: None,
            ayasofya_ici_texture: None,
        }
    }
}
