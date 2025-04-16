use crate::physics_engine::vector2d::Vector2D;

/// Represents an space or a scene that contains physics-simulated elements
pub struct PhysicsEngine {
    pub elements: Vec<Rigidbody>,
}

impl PhysicsEngine {
    pub fn pass_time(&mut self, time: f32) {
        
    }
}

// TODO: Implement a radius to it
// An element inside of the engine with gravity being applied
pub struct Rigidbody {
    pub position: Vector2D,
    pub mass: f64,
}

impl Rigidbody {
    pub fn gravity_acceleration_to(&self, target: Rigidbody) -> Vector2D {

        let distance_x = target.position.x - self.position.x;
        let distance_y = target.position.y - self.position.y;

        let distance = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

        const GRAVITATIONAL_CONSTANT: f64 = 6.67408 / (10u64.pow(11u32)) as f64;

        let gravity_force = (GRAVITATIONAL_CONSTANT * target.mass) / distance.powi(2);

        let acceleration_x = gravity_force * distance_x;
        let acceleration_y = gravity_force * distance_y;

        Vector2D { x: acceleration_x, y: acceleration_y }
    }
}