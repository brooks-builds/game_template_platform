use ggez::graphics::{DrawParam, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::GameState;

pub struct GameObject {
    location: Point2<f32>,
    velocity: Point2<f32>,
}

impl GameObject {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            location: Point2 { x, y },
            velocity: Point2 { x: 1.0, y: 0.0 },
        }
    }

    pub fn draw(&self, context: &mut Context, mesh: &Mesh) -> GameResult {
        ggez::graphics::draw(context, mesh, DrawParam::new().dest(self.location))
    }

    pub fn update(&mut self, gravity: f32) {
        self.velocity.y += gravity;
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;
    }
}
