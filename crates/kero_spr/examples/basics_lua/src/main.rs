use kero::prelude::*;
use kero_spr::SprModules;

fn main() -> Result<(), GameError> {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;

    kero::new_game()
        .with_default_logger()
        .with_title("Basics Lua")
        .with_size(1280, 720)
        .with_modules::<SprModules>()?
        .run_lua()
}
