use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};

use crate::draw_system::DrawSystem;
use crate::drawables::Drawables;
use crate::physics_system::PhysicsSystem;

use self::entity_state::EntityState;

pub mod builder;
pub mod entity_data;
pub mod entity_state;
pub mod entity_type;

#[derive(Debug)]
pub struct Entity {
    pub location: Vector2<f32>,
    pub width: f32,
    pub height: f32,
    draw_system: Option<Box<dyn DrawSystem>>,
    affected_by_gravity: bool,
    pub physics_system: Option<Box<dyn PhysicsSystem>>,
    pub collidable: bool,
    state: EntityState,
    pub id: u32,
}

impl Entity {
    /// Create a new entity with the default values and give it a specific ID
    /// ```
    /// use game_template_platform::entity::Entity;
    /// let mut entity = Entity::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the location of the entity. We are using a GGEZ Rect to store the location and size data together
    /// ```
    /// use game_template_platform::entity::Entity;
    /// let mut player = Entity::new();
    /// player.set_location(50.0, 65.0);
    /// assert_eq!(player.location.x, 50.0);
    /// assert_eq!(player.location.y, 65.0);
    /// ```
    pub fn set_location(&mut self, x: f32, y: f32) -> &mut Self {
        self.location.x = x;
        self.location.y = y;

        self
    }

    pub fn set_draw_system(&mut self, draw_system: Box<dyn DrawSystem>) -> &mut Self {
        self.draw_system = Some(draw_system);

        self
    }

    pub fn set_affected_by_gravity(&mut self) -> &mut Self {
        self.affected_by_gravity = true;
        self
    }

    pub fn set_physics_system(&mut self, physics_system: Box<dyn PhysicsSystem>) -> &mut Self {
        self.physics_system = Some(physics_system);
        self
    }

    pub fn set_collidable(&mut self, collidable: bool) -> &mut Self {
        self.collidable = collidable;
        self
    }

    /// set the width and height of the entity
    /// ```
    /// use game_template_platform::entity::Entity;
    /// let mut player = Entity::new();
    /// player.set_size(50.0, 150.0);
    /// assert_eq!(player.width, 50.0);
    /// assert_eq!(player.height, 150.0);
    /// ```
    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables, lag: f32) -> GameResult {
        if let Some(draw_system) = &self.draw_system {
            draw_system.draw(
                drawables,
                context,
                &self.location,
                (self.width, self.height),
                lag,
                &self.physics_system,
            )?;
        }

        Ok(())
    }

    /// Update the entity and return true if the location of the entity changed
    pub fn update(&mut self, gravity: &Vector2<f32>, collidable_others: Vec<Entity>) -> bool {
        let original_location = self.location;
        if let Some(physics_system) = &mut self.physics_system {
            // and we are not standing
            if self.affected_by_gravity {
                match self.state {
                    EntityState::None => physics_system.apply_force(gravity),
                    EntityState::Falling => physics_system.apply_force(gravity),
                    EntityState::Standing => {}
                }
            }

            // we might also want to pass in the gravity force, and then the update function will determine if the gravity force should be applied
            // We will also want to create and pass in an entity state here for updating.
            physics_system.update(
                &mut self.location,
                self.width,
                self.height,
                collidable_others,
                &mut self.state,
            );
        }
        original_location == self.location
    }
}

impl Default for Entity {
    fn default() -> Self {
        let location = Vector2::new(0.0, 0.0);
        let width = 0.0;
        let height = 0.0;
        let draw_system = None;
        let affected_by_gravity = false;
        let physics_system = None;
        let collidable = false;

        Self {
            location,
            width,
            height,
            draw_system,
            affected_by_gravity,
            physics_system,
            collidable,
            state: EntityState::None,
            id: 0,
        }
    }
}

impl Clone for Entity {
    fn clone(&self) -> Self {
        Self {
            location: self.location,
            width: self.width,
            height: self.height,
            draw_system: None,
            affected_by_gravity: self.affected_by_gravity,
            physics_system: None,
            collidable: self.collidable,
            state: self.state,
            id: self.id,
        }
    }
}
