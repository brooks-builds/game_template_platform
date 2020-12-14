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
            velocity: Point2 { x: 5.0, y: 0.0 },
        }
    }

    pub fn draw(&self, context: &mut Context, mesh: &Mesh) -> GameResult {
        ggez::graphics::draw(context, mesh, DrawParam::new().dest(self.location))
    }

    pub fn update(&mut self, gravity: f32, screen_size: (f32, f32)) {
        self.velocity.y += gravity;
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;

        self.hit_walls(screen_size);
    }

    fn hit_walls(&mut self, (width, height): (f32, f32)) {
        if self.location.x > width {
            self.location.x = width;
            self.velocity.x *= -1.0;
        } else if self.location.x < 0.0 {
            self.location.x = 0.0;
            self.velocity.x *= -1.0;
        }

        if self.location.y > height {
            self.location.y = height;
            self.velocity.y *= -1.0;
        } else if self.location.y < 0.0 {
            self.location.y = 0.0;
            self.velocity.y *= -1.0;
        }
    }
}
