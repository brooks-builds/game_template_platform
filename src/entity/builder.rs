use ggez::nalgebra::Vector2;

use crate::draw_system::DrawSystem;
use crate::physics_system::PhysicsSystem;
use crate::Entity;

use super::entity_state::EntityState;

pub struct EntityBuilder {
    next_id: u32,
    location: Vector2<f32>,
    width: f32,
    height: f32,
    draw_system: Option<Box<dyn DrawSystem>>,
    affected_by_gravity: bool,
    physics_system: Option<Box<dyn PhysicsSystem>>,
    collidable: bool,
    state: EntityState,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            location: Vector2::new(0.0, 0.0),
            width: 0.0,
            height: 0.0,
            draw_system: None,
            affected_by_gravity: false,
            physics_system: None,
            collidable: false,
            state: EntityState::None,
        }
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self
    }

    pub fn build(&mut self) -> Entity {
        let id = self.next_id;
        let entity = Entity {
            location: self.location,
            width: self.width,
            height: self.height,
            draw_system: self.draw_system.take(),
            affected_by_gravity: self.affected_by_gravity,
            physics_system: self.physics_system.take(),
            collidable: self.collidable,
            state: self.state,
            id,
        };
        self.reset();
        entity
    }

    pub fn location(&mut self, x: f32, y: f32) -> &mut Self {
        self.location.x = x;
        self.location.y = y;
        self
    }

    pub fn size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn draw_system(&mut self, draw_system: Box<dyn DrawSystem>) -> &mut Self {
        self.draw_system = Some(draw_system);
        self
    }

    pub fn reset(&mut self) {
        self.next_id += 1;
        self.location = Vector2::new(0.0, 0.0);
        self.width = 0.0;
        self.height = 0.0;
        self.draw_system = None;
        self.affected_by_gravity = false;
        self.collidable = false;
        self.state = EntityState::None;
    }

    pub fn affected_by_gravity(&mut self) -> &mut Self {
        self.affected_by_gravity = true;
        self
    }

    pub fn physics_system(&mut self, physics_system: Box<dyn PhysicsSystem>) -> &mut Self {
        self.physics_system = Some(physics_system);
        self
    }

    pub fn collidable(&mut self) -> &mut Self {
        self.collidable = true;
        self
    }

    pub fn state(&mut self, state: EntityState) -> &mut Self {
        self.state = state;
        self
    }
}

#[cfg(test)]
mod test {
    use ggez::nalgebra::Vector2;

    use crate::draw_system::player_draw_system::PlayerDrawSystem;
    use crate::physics_system::player_physics_system::PlayerPhysicsSystem;
    use crate::Entity;

    use super::*;

    #[test]
    fn entity_builder_should_create_a_new_entity_builder() {
        let entity_builder: EntityBuilder = EntityBuilder::new();
        assert_eq!(entity_builder.next_id, 0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn entity_builder() {
        let mut entity_builder = EntityBuilder::new();
        let basic_entity: Entity = entity_builder.create_entity().build();

        assert_eq!(entity_builder.next_id, 1);
        assert_eq!(basic_entity.location, Vector2::new(0.0, 0.0));
        assert_eq!(basic_entity.width, 0.0);
        assert_eq!(basic_entity.height, 0.0);
        assert!(matches!(basic_entity.draw_system, None));
        assert_eq!(basic_entity.affected_by_gravity, false);
        assert!(matches!(basic_entity.physics_system, None));
        assert_eq!(basic_entity.collidable, false);
        assert!(matches!(
            basic_entity.state,
            crate::entity::entity_state::EntityState::None
        ));
        assert_eq!(basic_entity.id, 0);
    }

    #[test]
    fn ci_test_create_entity_with_location() {
        let x = 42.0;
        let y = 24.0;
        let expected_location = Vector2::new(x, y);
        let mut entity_builder = EntityBuilder::new();
        let entity_with_location = entity_builder.create_entity().location(x, y).build();

        assert_eq!(entity_with_location.location, expected_location);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_create_entity_with_size() {
        let width = 42.0;
        let height = 24.0;
        let mut entity_builder = EntityBuilder::new();
        let entity_with_size = entity_builder.create_entity().size(width, height).build();

        assert_eq!(entity_with_size.width, width);
        assert_eq!(entity_with_size.height, height);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_create_entity_with_draw_system() {
        let draw_system = PlayerDrawSystem;
        let mut entity_builder = EntityBuilder::new();
        let entity = entity_builder
            .create_entity()
            .draw_system(Box::new(draw_system))
            .build();

        assert!(!matches!(entity.draw_system, None));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ci_test_reset_entity_builder_after_build() {
        let mut entity_builder = EntityBuilder::new();
        let basic_entity: Entity = entity_builder
            .create_entity()
            .location(15.0, 156.0)
            .size(25.0, 15.0)
            .draw_system(Box::new(PlayerDrawSystem))
            .affected_by_gravity()
            .physics_system(Box::new(PlayerPhysicsSystem::default()))
            .collidable()
            .state(EntityState::Standing)
            .build();

        assert_eq!(entity_builder.next_id, 1);
        assert_eq!(entity_builder.location, Vector2::new(0.0, 0.0));
        assert_eq!(entity_builder.width, 0.0);
        assert_eq!(entity_builder.height, 0.0);
        assert!(matches!(entity_builder.draw_system, None));
        assert_eq!(entity_builder.affected_by_gravity, false);
        assert!(entity_builder.physics_system.is_none());
        assert_eq!(entity_builder.collidable, false);
        assert!(matches!(entity_builder.state, EntityState::None));
    }

    #[test]
    fn ci_test_create_entity_affected_by_gravity() {
        let mut entity_builder = EntityBuilder::new();
        let entity = entity_builder.create_entity().affected_by_gravity().build();
        assert_eq!(entity.affected_by_gravity, true);
    }

    #[test]
    fn ci_test_entity_builder_should_create_an_entity_with_physics_system() {
        let mut entity_builder = EntityBuilder::new();
        let entity = entity_builder
            .create_entity()
            .physics_system(Box::new(PlayerPhysicsSystem::default()))
            .build();
        assert!(entity.physics_system.is_some());
    }

    #[test]
    fn ci_test_entity_builder_should_create_collidable_entity() {
        let mut entity_builder = EntityBuilder::new();
        let entity = entity_builder.create_entity().collidable().build();
        assert!(entity.collidable);
    }

    #[test]
    fn ci_test_entity_builder_should_create_entity_with_state() {
        let mut entity_builder = EntityBuilder::new();
        let entity = entity_builder
            .create_entity()
            .state(EntityState::Falling)
            .build();
        assert!(matches!(entity.state, EntityState::Falling));
    }
}
