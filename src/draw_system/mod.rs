use std::fmt::Debug;

use ggez::{Context, GameResult};

use crate::drawables::Drawables;
use crate::physics_system::PhysicsSystem;

pub mod platform_draw_system;
pub mod player_draw_system;

pub trait DrawSystem
where
    Self: Debug,
{
    fn draw(
        &self,
        drawables: &Drawables,
        context: &mut Context,
        location: &ggez::nalgebra::Vector2<f32>,
        size: (f32, f32),
        lag: f32,
        physics_system: &Option<Box<dyn PhysicsSystem>>,
    ) -> GameResult;
}
