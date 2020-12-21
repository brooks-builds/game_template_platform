pub mod player_physics_system;

use std::fmt::Debug;

use crate::entity::Entity;

pub trait PhysicsSystem
where
    Self: Debug,
{
    fn apply_force(&mut self, force: &ggez::nalgebra::Vector2<f32>);
    fn update(&mut self, location: &mut ggez::graphics::Rect, collidables: Vec<Entity>);
    fn get_velocity(&self) -> &ggez::nalgebra::Vector2<f32>;
}
