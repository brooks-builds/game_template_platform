use std::collections::HashMap;

use bbggez_ecs_world::component::{Component, ComponentDATA};
use bbggez_ecs_world::system::System;
use bbggez_ecs_world::World;
use ggez::event::EventHandler;
use ggez::graphics::{DrawMode, BLACK};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use graphics::{DrawParam, Mesh, MeshBuilder, WHITE};

pub struct GameState {
    world: World<Component>,
}

impl GameState {
    pub fn new(_context: &Context) -> GameResult<Self> {
        let mut world = World::new();
        world.register("location");
        world.register("size");

        world
            .create_component()
            .with(
                "location",
                Component::new(Box::new(Location { x: 150.0, y: 150.0 })),
            )
            .with("size", Component::new(Box::new(Size(10.0))));

        Ok(Self { world })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);

        graphics::present(context)
    }
}

struct DrawSystem;

impl System for DrawSystem<DATA: ComponentDATA> {
    fn run(&mut self, data: &mut HashMap<String, DATA>) {
        let locations = data.get("location").unwrap();
        let sizes = data.get("sizes").unwrap();

        for index in 0..locations.len() {
            let location = locations[index];
            let size = sizes[index];
            let point = Point2 {
                x: location.x,
                y: location.y,
            };
            // let mesh = MeshBuilder::new()
            //     .circle(DrawMode::fill(), &point, sizes[index], 0.1, WHITE)
            //     .build(context)
            //     .unwrap();
        }
    }
}

struct Location {
    x: f32,
    y: f32,
}

impl ComponentDATA for Location {}

struct Size(f32);

impl ComponentDATA for Size {}
