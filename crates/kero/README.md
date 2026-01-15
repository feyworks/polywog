![Kero](assets/header.png)

An approachable cross-platform framework for creating 2D games in either Rust, Lua, or both.

> ⚠️ <span style="color:red">**KERO IS CURRENTLY IN UNSTABLE ALPHA TESTING PHASE AND NOT FOR GENERAL USE**</span>

- [✅ Features](#-features)
- [💡 Getting started](#-getting-started)
  - [Install Rust](#install-rust)
  - [Clone the Kero repository](#clone-the-kero-repository)
  - [Create a new project](#create-a-new-project)
- [💃 Join the community](#-join-the-community)

## ✅ Features

Kero is a pure-code framework that programmers can use to code their games or even to build their
own game engines. It provides:

- 🖥️ a window, game loop, and rendering context out of the box and ready to go
- 🎮 mouse, keyboard, and gamepad input as well as virtual input mapping
- 🖼️ shaders, surfaces, textures, and other graphics resources
- 🖌️ a straightforward but powerful canvas-style drawing API
- 🧮 various math types for vectors, matrices, rotations, etc.
- 📐 geometry types for various shapes, overlap testing, extraction, raycasting, etc.
- 🎨 tools for working with colors, image encoding, decoding, and manipulation
- 🧳 texture packing and other techniques for rendering optimization
- 🦀 full access to Rust's speed, power, ecosystem, and pleasure of use
- 🌙 full Lua bindings if desired, with LuaLS type annotations

## 💡 Getting started

### Install Rust

If you don't already know or use Rust, you should first
[install it](https://rust-lang.org/tools/install/) and then follow the
[getting started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
tutorial at the very least. The whole Rust book is a very good learning resource.

If you need a code editor, Visual Studio Code has a
[Rust extension](https://code.visualstudio.com/docs/languages/rust) that is very widely
used and supported.

### Clone the Kero repository

In your chosen Rust project folder, clone the repository:

```console
cd ~/my_rust_folder
git clone https://github.com/feyworks/kero
```

With a local copy, you can now build the documentation:

```console
cd kero
cargo doc --open
```

### Create a new project

From the same root Rust project folder, now create a new binary project:

```console
cd ~/my_rust_folder
cargo new --bin my_game
cd my_game
```

Then, add `kero` and `env_logger` as dependencies.

```console
cargo add --git https://github.com/feyworks/kero kero
cargo add env_logger
```

Kero is not a Rust package yet so you have to add it directly from the repository.

Next, open `src/main.rs` and replace it with the following code:

```rust
use kero::prelude::*;

fn main() -> Result<(), GameError> {
    env_logger::init();

    // create a game, set some options, and then run it
    kero::new_game()
        .with_title("My Game")
        .with_size(1280, 720)
        .run::<MyGame>(())
}

pub struct MyGame {}

impl Game for MyGame {
    type Config = ();

    fn new(ctx: &Context, cfg: Self::Config) -> Result<Self, GameError>
    where
        Self: Sized,
    {
        // initialize your game state here, such as creating graphics resources, etc.
        Ok(Self {})
    }

    fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
        // perform your game logic here
        Ok(())
    }

    fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
        // perform your drawing code here
        Ok(())
    }
}
```

You can now run the game with:

```console
cargo run
```

From here, the journey is yours. Use `Context` to access the mouse, keyboard, window,
and graphics APIs. Browse `Draw` for a variety for drawing functions.

## 💃 Join the community

Join our [Discord](https://discord.gg/AYjNw9WHJa) to chat, get help, report bugs, and share what you're working on!

Check out our [{{TODO: Contributing}}]() page if you're interested in helping maintain and improve the
project.

Say hello to our mascot [{{TODO: MASCOT_NAME}}]().