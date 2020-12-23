use ggez::graphics::{draw, Color, DrawMode, DrawParam, MeshBuilder, Rect, WHITE};
use ggez::Context;

use crate::drawables::Drawables;
use crate::physics_system::PhysicsSystem;

use super::DrawSystem;

#[derive(Debug)]
pub struct PlatformDrawSystem {
    color: ggez::graphics::Color,
}

impl PlatformDrawSystem {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl DrawSystem for PlatformDrawSystem {
    fn draw(
        &self,
        drawables: &Drawables,
        context: &mut Context,
        location: &ggez::nalgebra::Vector2<f32>,
        (width, height): (f32, f32),
        _lag: f32,
        _physics_system: &Option<Box<dyn PhysicsSystem>>,
    ) -> ggez::GameResult {
        draw(
            context,
            &drawables.platform,
            DrawParam::new()
                .dest([location.x, location.y])
                .color(self.color),
        )?;

        let border = MeshBuilder::new()
            .rectangle(
                DrawMode::stroke(2.0),
                Rect::new(
                    location.x - width / 2.0,
                    location.y - height / 2.0,
                    width,
                    height,
                ),
                WHITE,
            )
            .build(context)?;

        draw(context, &border, DrawParam::new())?;

        Ok(())
    }
}
