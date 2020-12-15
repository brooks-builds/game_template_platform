use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::drawables::Drawables;
use crate::entity::Entity;

#[derive(Default)]
pub struct World {
    entities: Vec<Entity>,
    gravity: Point2<f32>,
}

impl World {
    pub fn set_gravity(mut self, gravity: f32) -> Self {
        self.gravity.y = gravity;

        self
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables) -> GameResult {
        self.entities
            .iter()
            .try_for_each(|entity| entity.draw(context, drawables))
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
        assert_eq!(world.gravity, 0.0);
        world = world.set_gravity(5.0);
        assert_eq!(world.gravity, 5.0);
    }
}
