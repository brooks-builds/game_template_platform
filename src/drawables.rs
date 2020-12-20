use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::world::World;

pub struct Drawables {
    pub player: Mesh,
    pub grid: Mesh,
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

        let grid = Self::create_grid(world, context)?;

        Ok(Self { player, grid })
    }

    fn create_grid(world: &World, context: &mut Context) -> GameResult<Mesh> {
        let color = Color::new(1.0, 1.0, 1.0, 0.5);
        let mut grid = &mut MeshBuilder::new();
        let vertical_count = world.width / world.unit_width;
        let horizontal_count = world.height / world.unit_height;

        for count in 0..vertical_count as usize {
            let x = world.unit_width * count as f32;
            let start_y = 0.0;
            let end_y = world.height;
            let points = [Point2 { x, y: start_y }, Point2 { x, y: end_y }];
            grid = grid.line(&points, 1.0, color)?;
        }

        for count in 0..horizontal_count as usize {
            let start_x = 0.0;
            let end_x = world.width;
            let y = world.unit_height * count as f32;
            let points = [Point2 { x: start_x, y }, Point2 { x: end_x, y }];
            grid = grid.line(&points, 1.0, color)?;
        }

        grid.build(context)
    }
}
