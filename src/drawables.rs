use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct Drawables {
    pub player: Mesh,
}

impl Drawables {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let player = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2 { x: 0.0, y: 0.0 },
                10.0,
                0.1,
                Color::new(0.4, 0.6, 0.9, 1.0),
            )
            .build(context)?;

        Ok(Self { player })
    }
}
