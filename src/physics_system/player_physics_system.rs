use ggez::nalgebra::Vector2;

use crate::entity::entity_state::EntityState;
use crate::entity::Entity;

use super::PhysicsSystem;

#[derive(Debug)]
pub struct PlayerPhysicsSystem {
    acceleration: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl PlayerPhysicsSystem {
    fn overlaps_with(
        &self,
        our_location: &Vector2<f32>,
        width: f32,
        height: f32,
        other: &Entity,
    ) -> bool {
        // is our right side to the right of the others left
        our_location.x + width / 2.0 > other.location.x - other.width / 2.0
        // and is our left side to the left of the others right
        && our_location.x - width / 2.0 < other.location.x + other.width / 2.0
        // and is our bottom below the others top
        && our_location.y + height / 2.0 > other.location.y - other.height / 2.0
        // and is our top above the others bottom?
        && our_location.y - height / 2.0 < other.location.y +  other.height / 2.0
    }
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

    fn update(
        &mut self,
        location: &mut ggez::nalgebra::Vector2<f32>,
        width: f32,
        height: f32,
        others: Vec<Entity>,
        state: &mut crate::entity::entity_state::EntityState,
    ) {
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

            if self.overlaps_with(location, width, height, other) {
                self.velocity *= 0.0;
                if location.y < other.location.y {
                    location.y = other.location.y - other.height / 2.0 - height / 2.0;
                    *state = EntityState::Standing;
                }
            }
        })
    }

    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }
}
