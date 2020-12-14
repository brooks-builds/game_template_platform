use ggez::event::EventHandler;
use ggez::graphics::BLACK;
use ggez::{Context, GameResult};

pub struct GameState {}

impl GameState {
    pub fn new(_context: &mut Context) -> GameResult<Self> {
        Ok(Self {})
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        ggez::graphics::clear(context, BLACK);

        ggez::graphics::present(context)
    }
}
