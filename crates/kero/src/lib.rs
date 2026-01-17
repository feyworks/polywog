//! ![Kero](https://raw.githubusercontent.com/feyworks/feyworks/refs/heads/main/crates/kero/assets/header.png)
//! 
//! An approachable cross-platform framework for creating 2D games in either Rust, Lua, or both.
//! 
//! ⚠️ **KERO IS CURRENTLY IN UNSTABLE ALPHA TESTING PHASE AND NOT FOR GENERAL USE**
//! 
//! - [✅ Features](#-features)
//! - [💡 Getting started](#-getting-started)
//! - [🔬 Alpha Testers](#-alpha-testers)
//! 
//! ## ✅ Features
//! 
//! Kero is a pure-code framework that programmers can use to code their games or even to build their
//! own game engines. It provides:
//! 
//! - 🖥️ a window, game loop, and rendering context out of the box and ready to go
//! - 🎮 mouse, keyboard, and gamepad input as well as virtual input mapping
//! - 🖼️ shaders, surfaces, textures, and other graphics resources
//! - 🖌️ a straightforward but powerful canvas-style drawing API
//! - 🧮 various math types for vectors, matrices, rotations, etc.
//! - 📐 geometry types for various shapes, overlap testing, extraction, raycasting, etc.
//! - 🎨 tools for working with colors, image encoding, decoding, and manipulation
//! - 🧳 texture packing and other techniques for rendering optimization
//! - 🦀 full access to Rust's speed, power, ecosystem, and pleasure of use
//! - 🌙 full Lua bindings if desired, with LuaLS type annotations
//! 
//! ## 💡 Getting started
//! 
//! There's no fancy setup required, Kero is just a normal crate. To create a new empty game project,
//! first create it and add `kero` as a dependency:
//! 
//! ```console
//! cargo new --bin my_game
//! cd my_game
//! cargo add kero
//! ```
//! 
//! Then, replace `src/main.rs` with the following:
//! 
//! ```no_run
//! use kero::prelude::*;
//! 
//! fn main() -> Result<(), GameError> {
//!     // create a game, set some options, and then run it
//!     kero::new_game()
//!         .with_default_logger()
//!         .with_title("My Game")
//!         .with_size(1280, 720)
//!         .run::<MyGame>(())
//! }
//! 
//! // store your game state and graphics resources here
//! pub struct MyGame {}
//! 
//! impl Game for MyGame {
//!     type Config = ();
//! 
//!     // initialize your game state here, such as creating graphics resources, etc.
//!     fn new(ctx: &Context, cfg: Self::Config) -> Result<Self, GameError>
//!     where
//!         Self: Sized,
//!     {
//!         Ok(Self {})
//!     }
//!
//!     // perform your game logic here
//!     fn update(&mut self, ctx: &Context) -> Result<(), GameError> {
//!         Ok(())
//!     }
//! 
//!     // perform your drawing code here
//!     fn render(&mut self, ctx: &Context, draw: &mut Draw) -> Result<(), GameError> {
//!         Ok(())
//!     }
//! }
//! ```
//! 
//! The [examples](https://github.com/feyworks/feyworks/tree/main/crates/kero/examples) folder has a
//! bunch of examples you can check out to see how different things are done.
//! 
//! ## 🔬 Alpha Testers
//! 
//! Thank you for helping test Kero! 💕 Please join our [**Discord**](https://discord.gg/AYjNw9WHJa)
//! where we are currently looking for feedback:
//! - from both casual, pro, and brand new Rust users
//! - first impressions, if what you expected it to do matched what it does
//! - naming conventions, API or organizational feedback
//! - features that are missing or you cannot find
//! - pics of anything you made! even if it’s basic rendering or movement etc.
//! - sharing your code so I can see how you’re using it/Rust
//! 
//! And if you think this is the kind of project you'd like to help out on, we're definitely interested
//! in having more contributors. It would be great if this could be polished up, stabilized, and turned
//! into a reliable game development tool for the Rust ecosystem.

pub mod core;
pub mod gfx;
pub mod input;
pub mod misc;
mod new_game;

#[cfg(feature = "lua")]
pub use fey_lua as lua;

#[cfg(feature = "lua")]
pub mod lua_modules;

#[doc(inline)]
pub use fey_color as color;

#[doc(inline)]
pub use fey_grid as grid;

#[doc(inline)]
pub use fey_guid as guid;

#[doc(inline)]
pub use fey_img as img;

#[doc(inline)]
pub use fey_math as math;

#[doc(inline)]
pub use fey_rand as rand;

pub use new_game::new_game;

///! Include all types and traits.
pub mod prelude {
    pub use crate::color::*;
    pub use crate::core::*;
    pub use crate::gfx::*;
    pub use crate::grid::*;
    pub use crate::guid::*;
    pub use crate::img::*;
    pub use crate::input::*;
    pub use crate::math::*;
    pub use crate::misc::*;
    pub use crate::rand::*;
}
