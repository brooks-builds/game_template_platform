use ggez::graphics::{
    apply_transformations, draw, pop_transform, push_transform, DrawParam, Mesh, Rect,
};
use ggez::mint::Point2;
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};

use crate::drawables::Drawables;
use crate::entity::Entity;

pub struct World {
    entities: Vec<Entity>,
    gravity: Vector2<f32>,
    pub width: f32,
    pub height: f32,
    pub unit_width: f32,
    pub unit_height: f32,
    dest: Point2<f32>,
}

impl World {
    pub fn set_gravity(mut self, gravity: f32) -> Self {
        self.gravity.y = gravity;

        self
    }

    pub fn set_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_unit_size(mut self, width: f32, height: f32) -> Self {
        self.unit_width = width;
        self.unit_height = height;
        self
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables, lag: f32) -> GameResult {
        push_transform(context, Some(DrawParam::new().dest(self.dest).to_matrix()));
        apply_transformations(context)?;
        draw(context, &drawables.grid, DrawParam::new())?;
        self.entities
            .iter()
            .try_for_each(|entity| entity.draw(context, drawables, lag))?;
        pop_transform(context);
        Ok(())
    }

    pub fn update(&mut self) {
        self.dest.x = -self.entities[0].location.x + 640.0;
        self.dest.y = -self.entities[0].location.y + 350.0;
        let gravity = &self.gravity;
        self.entities
            .iter_mut()
            .for_each(|entity| entity.update(gravity));
    }
}

impl Default for World {
    fn default() -> Self {
        let entities = vec![];
        let gravity = Vector2::new(0.0, 0.0);
        let width = 5000.0;
        let height = 5000.0;
        let unit_width = 1.0;
        let unit_height = 1.0;

        Self {
            entities,
            gravity,
            width,
            height,
            unit_width,
            unit_height,
            dest: Point2 { x: 0.0, y: 0.0 },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ci_test_add_entity_to_world() {
        let mut world = World::default();

        assert_eq!(world.entities.len(), 0);
        let entity = Entity::default();
        world.add_entity(entity);
        assert_eq!(world.entities.len(), 1);
    }

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
