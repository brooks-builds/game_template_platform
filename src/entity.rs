use ggez::graphics::{DrawParam, Mesh, Rect};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::draw_system::DrawSystem;
use crate::drawables::Drawables;
use crate::GameState;

#[derive(Debug)]
pub struct Entity {
    location: Rect,
    draw_system: Option<Box<dyn DrawSystem>>,
    affected_by_gravity: bool,
    physics_system: Option<Box<dyn PhysicsSystem>>,
}

impl Entity {
    pub fn set_location(mut self, x: f32, y: f32) -> Self {
        self.location.x = x;
        self.location.y = y;

        self
    }

    pub fn set_draw_system(mut self, draw_system: Box<dyn DrawSystem>) -> Self {
        self.draw_system = Some(draw_system);

        self
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables) -> GameResult {
        if let Some(draw_system) = &self.draw_system {
            draw_system.draw(drawables, context, &self.location)?;
        }

        Ok(())
    }

    pub fn update(&mut self, gravity: Point2<f32>) {
        if let Some(physics_system) = self.physics_system {
            if self.affected_by_gravity {
                physics_system.apply_force(gravity);
            }
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        let location = Rect::new(0.0, 0.0, 0.0, 0.0);
        let draw_system = None;
        let affected_by_gravity = false;

        Self {
            location,
            draw_system,
            affected_by_gravity,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::draw_system::player_draw_system::PlayerDrawSystem;

    use super::*;

    #[test]
    fn ci_test_set_location() {
        let mut entity = Entity::default();
        let mut location = Rect::new(0.0, 0.0, 0.0, 0.0);

        assert_eq!(entity.location, location);
        entity = entity.set_location(1.0, 2.0);
        location.x = 1.0;
        location.y = 2.0;
        assert_eq!(entity.location, location);
    }

    #[test]
    fn test_add_player_draw_system() {
        let mut entity = Entity::default();
        assert!(matches!(entity.draw_system, None));
        let player_draw_system = Box::new(PlayerDrawSystem);
        entity = entity.set_draw_system(player_draw_system);
        assert!(!matches!(entity.draw_system, None));
    }
}
