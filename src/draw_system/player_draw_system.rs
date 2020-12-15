use ggez::graphics::{draw, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::drawables::Drawables;

use super::DrawSystem;

#[derive(Debug)]
pub struct PlayerDrawSystem;

impl DrawSystem for PlayerDrawSystem {
    fn draw(&self, drawables: &Drawables, context: &mut Context, location: &Rect) -> GameResult {
        draw(
            context,
            &drawables.player,
            DrawParam::default().dest([location.x, location.y]),
        )
    }
}
