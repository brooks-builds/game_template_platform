pub mod cell;
pub mod gridv2;

use std::collections::HashMap;

use entity::builder::EntityBuilder;
use entity::entity_type;
use ggez::graphics::Rect;
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};
use gridv2::Grid;

use crate::camera::Camera;
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
    camera: Camera,
    entities: HashMap<u32, Entity>,
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
    pub fn build(&mut self, entity_builder: &mut EntityBuilder) {
        self.load_level(entity_builder);
    }

    pub fn add_entity(&mut self, entity: Entity) {
        if let Some(grid) = &mut self.grid {
            grid.insert(&entity);
            self.entities.insert(entity.id, entity);
        }
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables, lag: f32) -> GameResult {
        // push_transform(context, Some(DrawParam::new().dest(self.dest).to_matrix()));
        // apply_transformations(context)?;
        // draw(context, &drawables.grid, DrawParam::new())?;
        if let Some(grid) = &self.grid {
            let entities = grid.query(self.camera.as_rect());

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
            let entities = grid.get_all_entities_mut();
            let mut moved_entities = vec![];
            for entity in entities {
                // get collidable entities near the entity
                let old_location = entity.location;
                entity.update(gravity, collidable_entities.clone());
                let new_location = entity.location;
                if old_location != new_location {
                    // we need to tell the grid that an entity has been updated and might need to change cells
                    moved_entities.push((entity.id, old_location, new_location));
                }
            }

            moved_entities
                .iter()
                .for_each(|(id, old_location, new_location)| {
                    grid.move_entity(*id, *old_location, *new_location);
                })
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
    pub fn load_level(&mut self, entity_builder: &mut EntityBuilder) {
        let level = self.levels[self.current_level_index].clone();
        self.reset_grid(level.width, level.height);

        level
            .entity_datas
            .iter()
            .for_each(|entity_data| match entity_data.entity_type {
                entity_type::EntityType::Player => {}
                entity_type::EntityType::Platform => {
                    let platform = entity_builder
                        .create_entity()
                        .collidable()
                        .location(entity_data.x, entity_data.y)
                        .size(entity_data.width, entity_data.height)
                        .build();
                    self.add_entity(platform);
                }
            });
    }

    pub fn add_level(&mut self, level: Level) -> &mut Self {
        self.levels.push(level);
        self
    }

    pub fn set_camera(&mut self, camera: Camera) -> &mut Self {
        self.camera = camera;
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
            camera: Camera::default(),
            entities: HashMap::new(),
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

    #[test]
    fn ci_test_set_camera() {
        let mut world = World::default();
        let default_camera = Camera::default();
        assert_eq!(world.camera, default_camera);
        let camera = Camera::new(10.0, 15.0, 50.0, 50.0);
        let _updated_world: &mut World = world.set_camera(camera);
        assert_eq!(world.camera, camera);
    }
}
