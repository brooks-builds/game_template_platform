mod draw_system;
mod drawables;
mod entity;
mod world;

use draw_system::player_draw_system::PlayerDrawSystem;
use drawables::Drawables;
use entity::Entity;
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, Drawable, Mesh, MeshBuilder, BLACK};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use world::World;

pub struct GameState {
    world: World,
    drawables: Drawables,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let mut world = World::default().set_gravity(1.0);
        let drawables = Drawables::new(context)?;

        // create player
        let player = Entity::default()
            .set_location(50.0, 50.0)
            .set_draw_system(Box::new(PlayerDrawSystem));
        // add player to world
        world.add_entity(player);

        Ok(Self { world, drawables })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // update world
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        ggez::graphics::clear(context, BLACK);

        // draw the world
        self.world.draw(context, &self.drawables)?;

        ggez::graphics::present(context)
    }
}
