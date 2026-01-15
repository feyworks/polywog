![Kero](assets/header.png)

Kero is across-platform framework for creating 2D games in the [Rust](https://rust-lang.org/)
programming language. It contains everything you need to start making a game with no hassle, and
then gets out of your way. The rest is up to you!

- [⚠️ Alpha Testing](#️-alpha-testing)
- [✅ Features](#-features)
  - [🧱 Just the basics](#-just-the-basics)
  - [🌐 Cross platform](#-cross-platform)
  - [👩‍🎓 Create while you learn](#-create-while-you-learn)
- [🤔 Is this for me?](#-is-this-for-me)
  - [👍 Kero may be for you if you...](#-kero-may-be-for-you-if-you)
  - [👎 Kero is not for you if you...](#-kero-is-not-for-you-if-you)
  - [🔍 Want an alternative?](#-want-an-alternative)
- [💡 Getting started](#-getting-started)
  - [Install Rust](#install-rust)
  - [Clone the Kero repository](#clone-the-kero-repository)
  - [Create a new project](#create-a-new-project)
- [💃 Join the community](#-join-the-community)

## ⚠️ Alpha Testing

Kero is currently unreleased and in alpha testing stages, and is not recommended for use in
any professional capacity at this point in time. It is nearly feature complete, but needs a lot of
work to be polished up and stabilized. We need more examples, more polish, more complete
documentation, doc tests, unit tests, a finished website, and CI plus a release schedule.

If you are intested in the project and want to see it succeed and make its way into a stable
release, then trying it out, [joining the Discord](https://discord.gg/AYjNw9WHJa), and getting
involved is the best way to help us.

We're looking for:

- people to use it and just see how it feels! is it easy, hard, awkward?
- API/naming feedback and suggestions, conventions, etc.
- thoughts on project direction, feature cutoff, and roadmap
- contributors to help with tests/docs/linting
- ...from both experienced and new Rust coders!

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

### 🧱 Just the basics

The feature cutoff for Kero is very intentional, it is meant to isolate the ecosystem as a 2D
game development environment, but does not want to be too opinionated about what you are coding or
how you code it.

It is *just* low level enough that you could build a game directly on top of this, or an engine,
apps/tools to aid in development.

### 🌐 Cross platform

It is also designed to be a protective layer between you and cross-platform concerns such as
rendering, operating systems, input devices, or anything else that you want to *Just Work* and not
think about. If the core Rust libraries it is built on support a platform, so should Kero.

At the current time, Windows, macOS, and Linux are priority targets. Eventually, the goal is for
the framework to make its way onto game consoles as well.

### 👩‍🎓 Create while you learn

Finally, Kero aims to be a fun way to learn the language while also getting to do something
creative with it. It comes with a growing suite of examples and templates to make getting new
prototypes up and running quick and painless, so you can get right to coding the fun part.

We want users to meet Rust in a way that softens a lot of its restrictions that often frighten off
new users, but allow them access to its power when they become more comfortable with it.

Making games and prototypes is a cool way to learn a programming language, we think.

## 🤔 Is this for me?

### 👍 Kero may be for you if you...

- want to make weird and cool 2D indie games and prototypes
- want to learn Rust in a playful, creative way
- want a backend to build your own 2D engine on top of

### 👎 Kero is not for you if you...

- want a game engine with an editor and all the bells & whistles
- want to make a 3D or big AAA game with a huge team and make zillions of bucks
- want to make mobile or web games (not impossible, I just have no interest in supporting it)

### 🔍 Want an alternative?

You can think Kero as akin to frameworks like [Love2D](https://www.love2d.org/),
[MonoGame](https://monogame.net/), [SDL](https://www.libsdl.org/), or
[Raylib](https://www.raylib.com/). If you want a framework like this but in another language,
we recommend checking out those great projects.

If you want to stick with Rust but want a more opinionated but also more substantial project with
a larger community, definitely check out [Bevy](https://bevy.org/)! They're cool folks making
something very ambitious.

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