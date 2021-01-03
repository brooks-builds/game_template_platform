use ggez::{ContextBuilder, GameResult};

/// Initialize a ggez game, returning a tuple with a context and event loop
pub fn initialize_game(
    game_name: &str,
    author: &str,
) -> GameResult<(ggez::Context, ggez::event::EventsLoop)> {
    ContextBuilder::new(game_name, author).build()
}
