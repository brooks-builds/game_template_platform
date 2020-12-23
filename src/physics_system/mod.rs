pub mod player_physics_system;

use std::fmt::Debug;

use crate::entity::Entity;

pub trait PhysicsSystem
where
    Self: Debug,
{
    fn apply_force(&mut self, force: &ggez::nalgebra::Vector2<f32>);
    fn update(
        &mut self,
        location: &mut ggez::nalgebra::Vector2<f32>,
        width: f32,
        height: f32,
        others: Vec<Entity>,
        state: &mut crate::entity::entity_state::EntityState,
    );
    fn get_velocity(&self) -> &ggez::nalgebra::Vector2<f32>;
}
