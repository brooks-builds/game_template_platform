use ggez::nalgebra::Vector2;

use super::PhysicsSystem;

#[derive(Debug)]
pub struct PlayerPhysicsSystem {
    acceleration: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Default for PlayerPhysicsSystem {
    fn default() -> Self {
        let acceleration = Vector2::new(0.0, 0.0);
        let velocity = Vector2::new(0.0, 0.0);

        Self {
            acceleration,
            velocity,
        }
    }
}

impl PhysicsSystem for PlayerPhysicsSystem {
    fn apply_force(&mut self, force: &ggez::nalgebra::Vector2<f32>) {
        self.acceleration += force;
    }

    fn update(&mut self, location: &mut ggez::graphics::Rect) {
        self.velocity += self.acceleration;
        location.x += self.velocity.x;
        location.y += self.velocity.y;
        self.acceleration *= 0.0;
    }
}
