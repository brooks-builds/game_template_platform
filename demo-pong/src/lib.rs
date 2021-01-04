use std::collections::HashMap;

use bbggez_ecs_world::component::ComponentData;
use bbggez_ecs_world::World;
use ggez::event::EventHandler;
use ggez::graphics::{DrawMode, BLACK};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use graphics::{DrawParam, Mesh, MeshBuilder, WHITE};

pub struct GameState {
    world: World,
}

impl GameState {
    pub fn new(_context: &Context) -> GameResult<Self> {
        let mut world = World::new();
        world.register("location");
        world.register("size");

        world
            .create_component()
            .with("location", ComponentData::Point { x: 250.0, y: 150.0 })
            .with("size", ComponentData::Radius(10.0));

        Ok(Self { world })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        let locations = self.world.query("location").unwrap();
        let sizes = self.world.query("size").unwrap();

        for index in 0..self.world.length() {
            let mesh = MeshBuilder::new()
                .circle(
                    DrawMode::fill(),
                    Point2 {
                        x: locations[index].get_point().unwrap().0,
                        y: locations[index].get_point().unwrap().1,
                    },
                    sizes[index].get_radius().unwrap(),
                    0.1,
                    WHITE,
                )
                .build(context)?;
            graphics::draw(context, &mesh, DrawParam::new())?;
        }

        graphics::present(context)
    }
}
