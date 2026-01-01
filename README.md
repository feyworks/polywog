# 🐸 Polywog

Polywog is across-platform framework for creating 2D games in the [Rust](https://rust-lang.org/)
programming language. It contains everything you need to start making a game with little hassle,
but little more than that. The rest is up to you!

## ✅ Features

Instead of a full-featured engine and editor like *Unity* or *Unreal*, Polywog is a pure-code
framework that programmers can use to code their games in or even use build their own game engines.
It provides:

- 🖥️ a window, game loop, and rendering context out of the box and ready to go
- 🎮 mouse, keyboard, and gamepad input as well as virtual input mapping
- 🖼️ shaders, surfaces, textures, and other graphics resources
- 🖌️ a straightforward but powerful canvas-style drawing API
- 🧮 various math types for vectors, matrices, rotations, etc.
- 📐 geometry types for various shapes, overlap testing, extraction, raycasting, etc. 
- 🎨 tools for working with colors, image encoding, decoding, and manipulation
- 🧳 texture packing and other techniques for rendering optimization
- 🦀 and of course, full access to Rust's speed, power, ecosystem, and pleasure of use

You can think of it as kin to frameworks like [Love2D](https://www.love2d.org/),
[MonoGame](https://monogame.net/), and [Phaser](https://phaser.io/).

## 🧱 Just the basics

The feature cutoff for Polywog is is very intentional, it is meant to isolate the ecosystem as a 2D
game development environment, but does not want to be too opinionated about what you are coding or
how you code it.

It is *just* low level enough that you could build a game directly on top of this, or an engine,
apps/tools to aid in development.

## 🌐 Cross platform

It is also designed to be a protective layer between you and cross-platform concerns such as
rendering, operating systems, input devices, or anything else that you want to *Just Work* and not
think about. If the core Rust libraries it is built on support a platform, so should Polywog.

At the current time, Windows, macOS, and Linux are priority targets. Eventually, the goal is for
the framework to make its way onto game consoles as well.

## 👩‍🎓 Create while you learn

Finally, Polywog is also designed to be a tool to help onboard people onto using Rust. It should be
a fun way to learn the language while also getting to do something creative with it. Because of this
goal, the framework sort of “hides” the complexity of many of its systems at first glance, but does
not forbid users from accessing many of its lower-level or type-generic elements when they want to.

We want users to user Rust in a way that softens a lot of its restrictions that often frighten off
new users, but allow them access to its power when they become more comfortable with it.

Making games and prototypes is a cool way to learn a programming language, we think.