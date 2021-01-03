use bbggez_utilities::initialize::initialize_game;
use demo_pong::GameState;
use ggez::{event, GameResult};

fn main() -> GameResult<()> {
    let (context, event_loop) = &mut initialize_game("demo pong", "brookzerker")?;
    let game_state = &mut GameState::new(context)?;

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => eprintln!("Game crashed due to error: {}", error),
    }

    Ok(())
}
