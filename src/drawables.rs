use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect, WHITE};
use ggez::{Context, GameResult};

use crate::config::Config;
use crate::world::World;

pub struct Drawables {
    pub player: Mesh,
    pub platform: Mesh,
}

impl Drawables {
    pub fn new(context: &mut Context, world: &World, config: &Config) -> GameResult<Self> {
        let player = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    -config.player_width / 2.0,
                    -config.player_height / 2.0,
                    config.player_width,
                    config.player_height,
                ),
                Color::new(0.1, 0.1, 1.0, 1.0),
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
