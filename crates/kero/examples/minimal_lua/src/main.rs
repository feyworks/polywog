use kero::prelude::*;

fn main() -> Result<(), GameError> {
    // run kero from this directory
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;

    // create a game, set some options, and then run it
    kero::new_game()
        .with_default_logger()
        .with_title("Minimal")
        .with_size(1280, 720)
        //.with_module::<MyModule>()
        .run_lua()

    // the game code is now handed over to lua/Main.lua
    // you can add your own Rust modules that can be loaded by `require`
}
