use crate::world::WorldState;
use tetra::graphics::Rectangle;
use tetra::math::Vec2;

/// Circular collider for NPCs
pub struct CircleCollider {
    pub pos: Vec2<f32>,
    pub radius: f32,
}

/// Rectangular collider for walls/doors
pub struct RectCollider {
    pub rect: Rectangle,
}

pub fn check_collision(pos: Vec2<f32>, radius: f32, world: &WorldState) -> bool {
    // Check circle colliders (NPCs)
    let circle_colliders = get_circle_colliders(world);
    for collider in &circle_colliders {
        let dx = pos.x - collider.pos.x;
        let dy = pos.y - collider.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < (radius + collider.radius) {
            return true;
        }
    }
    
    // Check rect colliders (walls)
    let rect_colliders = get_rect_colliders(world);
    for collider in &rect_colliders {
        if circle_rect_collision(pos, radius, &collider.rect) {
            return true;
        }
    }
    
    false
}

/// Check collision between a circle and a rectangle
fn circle_rect_collision(circle_pos: Vec2<f32>, radius: f32, rect: &Rectangle) -> bool {
    // Find the closest point on the rectangle to the circle center
    let closest_x = circle_pos.x.max(rect.x).min(rect.x + rect.width);
    let closest_y = circle_pos.y.max(rect.y).min(rect.y + rect.height);
    
    // Calculate distance from circle center to closest point
    let dx = circle_pos.x - closest_x;
    let dy = circle_pos.y - closest_y;
    let distance_sq = dx * dx + dy * dy;
    
    distance_sq < radius * radius
}

fn get_circle_colliders(world: &WorldState) -> Vec<CircleCollider> {
    let mut colliders = Vec::new();

    match world.current_stage {
        1 => {
            // Sans
            colliders.push(CircleCollider {
                pos: world.sans_pos,
                radius: 40.0,
            });
            // MusicBox
            colliders.push(CircleCollider {
                pos: world.musicbox_pos,
                radius: 30.0,
            });
        }
        2 => {
            // Rarity
            if world.rarity_alive {
                colliders.push(CircleCollider {
                    pos: world.rarity_pos,
                    radius: 40.0,
                });
            }
            // Gaster
            colliders.push(CircleCollider {
                pos: world.gaster_pos,
                radius: 40.0,
            });
        }
        4 => {
            // Eilish
            colliders.push(CircleCollider {
                pos: world.eilish_pos,
                radius: 40.0,
            });
        }
        _ => {}
    }

    colliders
}

fn get_rect_colliders(world: &WorldState) -> Vec<RectCollider> {
    let mut colliders = Vec::new();

    if world.current_stage == 3 {
        // Ayasofya: Kapının dışındaki duvarlar
        // Sol duvar
        colliders.push(RectCollider {
            rect: Rectangle::new(0.0, 0.0, 300.0, 600.0),
        });
        // Sağ duvar
        colliders.push(RectCollider {
            rect: Rectangle::new(500.0, 0.0, 300.0, 600.0),
        });
        // Üst duvar (kapının üstü)
        colliders.push(RectCollider {
            rect: Rectangle::new(300.0, 0.0, 200.0, 150.0),
        });
    }

    colliders
}
