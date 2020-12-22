use ggez::nalgebra::Vector2;

use crate::entity::Entity;

use super::PhysicsSystem;

#[derive(Debug)]
pub enum PlayerState {
    Falling,
    Standing,
}

#[derive(Debug)]
pub struct PlayerPhysicsSystem {
    acceleration: Vector2<f32>,
    velocity: Vector2<f32>,
    pub state: PlayerState,
}

impl Default for PlayerPhysicsSystem {
    fn default() -> Self {
        let acceleration = Vector2::new(0.0, 0.0);
        let velocity = Vector2::new(0.0, 0.0);

        Self {
            acceleration,
            velocity,
            state: PlayerState::Falling,
        }
    }
}

impl PhysicsSystem for PlayerPhysicsSystem {
    fn apply_force(&mut self, force: &ggez::nalgebra::Vector2<f32>) {
        self.acceleration += force;
    }

    fn update(&mut self, location: &mut ggez::graphics::Rect, others: Vec<Entity>) {
        self.velocity += self.acceleration;
        if self.velocity.y > 10.0 {
            self.velocity.y = 10.0;
        } else if self.velocity.y < -10.0 {
            self.velocity.y = -10.0;
        }
        location.x += self.velocity.x;
        location.y += self.velocity.y;
        self.acceleration *= 0.0;

        others.iter().for_each(|other| {
            if !other.collidable {
                return;
            }

            if location.overlaps(&other.location) {
                self.velocity *= 0.0;
                location.y = other.location.y - other.location.h / 2.0 - location.h / 2.0;
                self.state = PlayerState::Standing;
            }
        })
    }

    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }
}
