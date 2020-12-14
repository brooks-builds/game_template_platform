use game_template_platform::GameState;
use ggez::conf::{Backend, WindowMode};
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let window_mode = WindowMode::default()
        .dimensions(1280.0, 720.0)
        .maximized(true)
        .resizable(true);
    let backend = Backend::default().version(3, 1).gles();
    let (context, event_loop) = &mut match ContextBuilder::new("Platform Template", "Brooks Patton")
        .window_mode(window_mode)
        .backend(backend)
        .build()
    {
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
