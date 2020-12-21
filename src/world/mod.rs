pub mod cell;
pub mod grid;

use ggez::graphics::{draw, DrawParam, Rect};
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};
use grid::Grid;

use crate::drawables::Drawables;
use crate::entity::Entity;

pub struct World {
    grid: Grid,
    gravity: Vector2<f32>,
    pub width: f32,
    pub height: f32,
    pub unit_width: f32,
    pub unit_height: f32,
}

impl World {
    pub fn set_gravity(mut self, gravity: f32) -> Self {
        self.gravity.y = gravity;

        self
    }

    pub fn set_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self.reset_grid();
        self
    }

    pub fn set_unit_size(mut self, width: f32, height: f32) -> Self {
        self.unit_width = width;
        self.unit_height = height;
        self.reset_grid();
        self
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.grid.insert(entity);
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables, lag: f32) -> GameResult {
        // push_transform(context, Some(DrawParam::new().dest(self.dest).to_matrix()));
        // apply_transformations(context)?;
        // draw(context, &drawables.grid, DrawParam::new())?;
        let entities = self.grid.query(Rect::new(0.0, 0.0, 1280.0, 720.0));
        entities
            .iter()
            .try_for_each(|entity| entity.draw(context, drawables, lag))?;
        // pop_transform(context);
        Ok(())
    }

    pub fn update(&mut self) {
        let gravity = &self.gravity;
        let collidable_entities = self.grid.get_all_cloned();
        let mut entities = self.grid.get_all_entities_mut();
        entities
            .iter_mut()
            .for_each(|entity| entity.update(gravity, collidable_entities.clone()));
    }

    fn reset_grid(&mut self) {
        let grid = Grid::new(self.width, self.height, self.unit_width, self.unit_height);
        self.grid = grid;
    }
}

impl Default for World {
    fn default() -> Self {
        let gravity = Vector2::new(0.0, 0.0);
        let width = 10.0;
        let height = 10.0;
        let unit_width = 10.0;
        let unit_height = 10.0;
        let grid = Grid::new(width, height, unit_width, unit_height);

        Self {
            grid,
            gravity,
            width,
            height,
            unit_width,
            unit_height,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_add_gravity_to_the_world() {
        let mut world: World = World::default();
        let mut gravity = Vector2::new(0.0, 0.0);
        assert_eq!(world.gravity, gravity);
        gravity.y = 5.0;
        world = world.set_gravity(5.0);
        assert_eq!(world.gravity, gravity);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_set_world_size() {
        let mut world: World = World::default();
        assert_eq!(world.width, 5000.0);
        assert_eq!(world.height, 5000.0);
        world = world.set_size(10_000.0, 7_000.0);
        assert_eq!(world.width, 10_000.0);
        assert_eq!(world.height, 7_000.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_set_world_unit_size() {
        let mut world = World::default();
        assert_eq!(world.unit_width, 1.0);
        assert_eq!(world.unit_height, 1.0);
        world = world.set_unit_size(50.0, 75.0);
        assert_eq!(world.unit_width, 50.0);
        assert_eq!(world.unit_height, 75.0);
    }
}
