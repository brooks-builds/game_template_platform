pub mod cell;
pub mod grid;

use entity::entity_type;
use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};
use grid::Grid;

use crate::draw_system::platform_draw_system::PlatformDrawSystem;
use crate::drawables::Drawables;
use crate::entity::{self, Entity};
use crate::level::Level;

pub struct World {
    grid: Option<Grid>,
    gravity: Vector2<f32>,
    pub width: f32,
    pub height: f32,
    pub unit_width: f32,
    pub unit_height: f32,
    levels: Vec<Level>,
    current_level_index: usize,
}

impl World {
    /// Create a new world with the default settings
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_gravity(&mut self, gravity: f32) -> &mut Self {
        self.gravity.y = gravity;

        self
    }

    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_unit_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.unit_width = width;
        self.unit_height = height;
        self
    }

    /// Load the level which is the final step in creating the world. We are now ready to start the game.
    pub fn build(&mut self) {
        self.load_level();
    }

    pub fn add_entity(&mut self, entity: Entity) {
        if let Some(grid) = &mut self.grid {
            grid.insert(entity);
        }
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables, lag: f32) -> GameResult {
        // push_transform(context, Some(DrawParam::new().dest(self.dest).to_matrix()));
        // apply_transformations(context)?;
        // draw(context, &drawables.grid, DrawParam::new())?;
        if let Some(grid) = &self.grid {
            let entities = grid.query(Rect::new(0.0, 0.0, 1280.0, 720.0));
            entities
                .iter()
                .try_for_each(|entity| entity.draw(context, drawables, lag))?;
        }
        // pop_transform(context);
        Ok(())
    }

    pub fn update(&mut self) {
        let gravity = &self.gravity;
        if let Some(grid) = &mut self.grid {
            let collidable_entities = grid.get_all_cloned();
            let mut entities = grid.get_all_entities_mut();
            entities
                .iter_mut()
                .for_each(|entity| entity.update(gravity, collidable_entities.clone()));
        }
    }

    pub fn reset_grid(&mut self, width: f32, height: f32) {
        let grid = Grid::new(width, height, self.unit_width, self.unit_height);
        self.grid = Some(grid);
    }

    /// Load a new level into the world.
    /// - reset the grid
    /// - create platforms and put them into the grid
    /// - load items and put into the grid
    /// - load enemies and put into the grid
    /// - load player and put into grid
    pub fn load_level(&mut self) {
        let level = self.levels[self.current_level_index].clone();
        self.reset_grid(level.width, level.height);

        level
            .entity_datas
            .iter()
            .for_each(|entity_data| match entity_data.entity_type {
                entity_type::EntityType::Player => {}
                entity_type::EntityType::Platform => {
                    let mut platform = Entity::new();
                    platform
                        .set_collidable(true)
                        .set_draw_system(Box::new(PlatformDrawSystem::new(entity_data.color)))
                        .set_location(entity_data.x, entity_data.y)
                        .set_size(entity_data.width, entity_data.height);
                    self.add_entity(platform);
                }
            });
    }

    pub fn add_level(&mut self, level: Level) -> &mut Self {
        self.levels.push(level);
        self
    }
}

impl Default for World {
    fn default() -> Self {
        let gravity = Vector2::new(0.0, 0.0);
        let width = 5000.0;
        let height = 5000.0;
        let unit_width = 1.0;
        let unit_height = 1.0;

        Self {
            grid: None,
            gravity,
            width,
            height,
            unit_width,
            unit_height,
            levels: vec![],
            current_level_index: 0,
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
        world.set_gravity(5.0);
        assert_eq!(world.gravity, gravity);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_set_world_size() {
        let mut world: World = World::default();
        assert_eq!(world.width, 5000.0);
        assert_eq!(world.height, 5000.0);
        world.set_size(10_000.0, 7_000.0);
        assert_eq!(world.width, 10_000.0);
        assert_eq!(world.height, 7_000.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_set_world_unit_size() {
        let mut world = World::default();
        assert_eq!(world.unit_width, 1.0);
        assert_eq!(world.unit_height, 1.0);
        world.set_unit_size(50.0, 75.0);
        assert_eq!(world.unit_width, 50.0);
        assert_eq!(world.unit_height, 75.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_add_level() {
        let mut world = World::default();
        assert_eq!(world.levels.len(), 0);
        let new_level = Level::new(100.0, 100.0, vec![]);
        world.add_level(new_level);
        assert_eq!(world.levels[0].width, 100.0);
        assert_eq!(world.current_level_index, 0);
    }
}
