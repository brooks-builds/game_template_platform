mod camera;
mod config;
mod draw_system;
mod drawables;
pub mod entity;
mod level;
mod physics_system;
mod world;

use camera::Camera;
use config::Config;
use draw_system::player_draw_system::PlayerDrawSystem;
use drawables::Drawables;
use entity::builder::EntityBuilder;
use entity::entity_data::EntityData;
use entity::entity_type::EntityType;
pub use entity::Entity;
use ggez::event::EventHandler;
use ggez::graphics::{Color, BLACK};
use ggez::timer::check_update_time;
use ggez::{Context, GameResult};
use level::Level;
use physics_system::player_physics_system::PlayerPhysicsSystem;
use world::World;

pub struct GameState {
    world: World,
    drawables: Drawables,
    target_update_fps: u32,
    entity_builder: EntityBuilder,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let config = Config::default();
        let level = Level::new(
            5000.0,
            5000.0,
            vec![
                EntityData::new(
                    config.player_start_x,
                    500.0,
                    config.world_unit_width,
                    config.world_unit_height,
                    Color::new(0.0, 1.0, 0.0, 1.0),
                    EntityType::Platform,
                ),
                EntityData::new(
                    500.0,
                    500.0,
                    config.world_unit_width,
                    config.world_unit_height,
                    Color::new(1.0, 0.0, 0.0, 1.0),
                    EntityType::Platform,
                ),
            ],
        );
        let camera = Camera::new(0.0, 0.0, 1280.0, 720.0);
        let mut world = World::new();
        let mut entity_builder = EntityBuilder::new();
        world
            .set_gravity(config.gravity_force)
            .set_size(config.world_width, config.world_height)
            .set_unit_size(config.world_unit_width, config.world_unit_height)
            .add_level(level)
            .set_camera(camera)
            .build(&mut entity_builder);
        let drawables = Drawables::new(context, &world, &config)?;
        let target_update_fps = config.target_update_fps;

        let player = entity_builder
            .create_entity()
            .location(config.player_start_x, config.player_start_y)
            .size(config.player_width, config.player_height)
            .draw_system(Box::new(PlayerDrawSystem))
            .affected_by_gravity()
            .physics_system(Box::new(PlayerPhysicsSystem::default()))
            .build();
        world.add_entity(player);

        Ok(Self {
            world,
            drawables,
            target_update_fps,
            entity_builder,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while check_update_time(context, self.target_update_fps) {
            self.world.update();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        ggez::graphics::clear(context, BLACK);

        let lag = ggez::timer::remaining_update_time(context).as_secs_f32()
            * self.target_update_fps as f32;
        self.world.draw(context, &self.drawables, lag)?;

        ggez::graphics::present(context)
    }
}
