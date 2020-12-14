use game_template_platform::GameState;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (context, event_loop) =
        &mut match ContextBuilder::new("Platform Template", "Brooks Patton").build() {
            Ok((context, event_loop)) => (context, event_loop),
            Err(error) => panic!(error),
        };

    let game_state = &mut GameState::new(context)?;

    match event::run(context, event_loop, game_state) {
        Ok(_) => println!("Thanks for playing!"),
        Err(error) => println!("Error occurred: {}", error),
    };

    Ok(())
}
