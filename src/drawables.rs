use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::world::World;

pub struct Drawables {
    pub player: Mesh,
    pub platform: Mesh,
}

impl Drawables {
    pub fn new(context: &mut Context, world: &World) -> GameResult<Self> {
        let player = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2 { x: 0.0, y: 0.0 },
                10.0,
                0.1,
                Color::new(0.4, 0.6, 0.9, 1.0),
            )
            .build(context)?;

        let platform = Self::create_platform(world, context)?;

        Ok(Self { player, platform })
    }

    fn create_platform(world: &World, context: &mut Context) -> GameResult<Mesh> {
        let width = world.unit_width;
        let height = world.unit_height;

        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(-width / 2.0, -height / 2.0, width, height),
                WHITE,
            )
            .build(context)
    }
}
