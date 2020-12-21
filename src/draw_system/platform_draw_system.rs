use ggez::graphics::{draw, Color, DrawMode, DrawParam, MeshBuilder, WHITE};

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
        drawables: &crate::drawables::Drawables,
        context: &mut ggez::Context,
        location: &ggez::graphics::Rect,
        _lag: f32,
        _physics_system: &Option<Box<dyn crate::physics_system::PhysicsSystem>>,
    ) -> ggez::GameResult {
        draw(
            context,
            &drawables.platform,
            DrawParam::new()
                .dest([location.x, location.y])
                .color(self.color),
        )?;

        Ok(())
    }
}
