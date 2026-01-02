use tetra::math::Vec2;
use crate::world::WorldState;

pub struct Collider {
    pub pos: Vec2<f32>,
    pub radius: f32,
}

pub fn check_collision(pos: Vec2<f32>, radius: f32, world: &WorldState) -> bool {
    let colliders = get_colliders(world);
    for collider in colliders {
        let dx = pos.x - collider.pos.x;
        let dy = pos.y - collider.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance < (radius + collider.radius) {
            return true;
        }
    }
    false
}

fn get_colliders(world: &WorldState) -> Vec<Collider> {
    let mut colliders = Vec::new();
    
    match world.current_stage {
        1 => {
            // Sans
            colliders.push(Collider {
                pos: world.sans_pos,
                radius: 50.0,
            });
            // MusicBox
            colliders.push(Collider {
                pos: world.musicbox_pos,
                radius: 40.0,
            });
        }
        2 => {
            // Rarity
            if world.rarity_alive {
                colliders.push(Collider {
                    pos: world.rarity_pos,
                    radius: 50.0,
                });
            }
            // Gaster (maybe?) - User didn't have collision for Gaster in original code, but let's add it if needed.
            // Original code only checked distance for interaction.
        }
        3 => {
            // Ayasofya walls?
        }
        4 => {
            // Eilish
            colliders.push(Collider {
                pos: world.eilish_pos,
                radius: 50.0,
            });
        }
        _ => {}
    }
    
    colliders
}
