mod game_object;

use game_object::GameObject;
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, BLACK};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub struct GameState {
    circle: Mesh,
    circles: Vec<GameObject>,
    gravity: f32,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let circle = MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2 { x: 0.0, y: 0.0 },
                5.0,
                0.1,
                Color::new(0.8, 0.3, 0.5, 1.0),
            )
            .build(context)?;
        Ok(Self {
            circle,
            circles: vec![],
            gravity: 1.0,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        if ggez::timer::ticks(context) % 200 == 0 {
            let fps = ggez::timer::fps(context);
            if fps > 59.0 {
                self.circles.push(GameObject::new(50.0, 50.0));
            } else if fps < 59.0 {
                self.circles.pop();
            }
            println!("fps: {} - circle count: {}", fps, self.circles.len());
        }

        let screen_size = ggez::graphics::drawable_size(context);
        while ggez::timer::check_update_time(context, 30) {
            let gravity = self.gravity;
            self.circles
                .iter_mut()
                .for_each(|circle| circle.update(gravity, screen_size));
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        ggez::graphics::clear(context, BLACK);

        for circle in &self.circles {
            circle.draw(context, &self.circle)?;
        }

        ggez::graphics::present(context)
    }
}
