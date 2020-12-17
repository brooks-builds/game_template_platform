mod config;
mod draw_system;
mod drawables;
mod entity;
mod physics_system;
mod world;

use config::Config;
use draw_system::player_draw_system::PlayerDrawSystem;
use drawables::Drawables;
use entity::Entity;
use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::timer::check_update_time;
use ggez::{Context, GameResult};
use physics_system::player_physics_system::PlayerPhysicsSystem;
use world::World;

pub struct GameState {
    world: World,
    drawables: Drawables,
    target_update_fps: u32,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let config = Config::default();
        let mut world = World::default().set_gravity(config.gravity_force);
        let drawables = Drawables::new(context)?;
        let target_update_fps = config.target_update_fps;

        // create player
        let player = Entity::default()
            .set_location(50.0, 50.0)
            .set_draw_system(Box::new(PlayerDrawSystem))
            .set_affected_by_gravity()
            .set_physics_system(Box::new(PlayerPhysicsSystem::default()));
        // add player to world
        world.add_entity(player);

        Ok(Self {
            world,
            drawables,
            target_update_fps,
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
