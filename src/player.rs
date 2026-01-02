use tetra::graphics::Texture;
use tetra::math::Vec2;
use crate::defs::Direction;

pub struct PlayerState {
    pub pos: Vec2<f32>,
    pub direction: Direction,
    pub health: f32,
    pub outfit: u8, // 0: None, 1: Fes, 2: Takke
    
    // Textures
    pub texture_front: Option<Texture>,
    pub texture_left: Option<Texture>,
    pub texture_right: Option<Texture>,
    pub texture_fes: Option<Texture>,
    pub texture_takke: Option<Texture>,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(400.0, 300.0),
            direction: Direction::Front,
            health: 100.0,
            outfit: 0,
            texture_front: None,
            texture_left: None,
            texture_right: None,
            texture_fes: None,
            texture_takke: None,
        }
    }
}
