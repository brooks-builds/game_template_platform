use ggez::graphics::{draw, DrawParam, Rect};
use ggez::{Context, GameResult};

use crate::drawables::Drawables;
use crate::physics_system::PhysicsSystem;

use super::DrawSystem;

#[derive(Debug)]
pub struct PlayerDrawSystem;

impl DrawSystem for PlayerDrawSystem {
    fn draw(
        &self,
        drawables: &Drawables,
        context: &mut Context,
        location: &Rect,
        lag: f32,
        physics_system: &Option<Box<dyn PhysicsSystem>>,
    ) -> GameResult {
        let mut x = location.x;
        let mut y = location.y;
        if let Some(physics_system) = physics_system {
            let part_velocity = physics_system.get_velocity() * lag;
            x += part_velocity.x;
            y += part_velocity.y;
        }
        draw(
            context,
            &drawables.player,
            DrawParam::default().dest([x, y]),
        )
    }
}
