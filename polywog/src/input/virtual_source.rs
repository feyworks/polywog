use crate::core::Context;
use crate::input::{Gamepad, Gamepads, Keyboard};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

/// Input source for a virtual input.
///
/// Multiple virtual buttons, axes, and sticks can all share an input source. This is used to
/// tell the inputs where to get their gamepad input from. You can choose to have input read from
/// a specific gamepad or whatever the last active gamepad happens to be.
#[derive(Clone)]
pub struct VirtualSource(Rc<Inner>);

impl Debug for VirtualSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GamepadSource").finish_non_exhaustive()
    }
}

impl PartialEq for VirtualSource {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for VirtualSource {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Rc::as_ptr(&self.0).partial_cmp(&Rc::as_ptr(&other.0))
    }
}

struct Inner {
    pub keyboard: Keyboard,
    pub gamepads: Gamepads,
    pub selector: RefCell<GamepadSelector>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GamepadSelector {
    LastActive,
    Specific(Option<Gamepad>),
}

impl VirtualSource {
    /// Create a new input source.
    pub fn new(ctx: &Context, selector: GamepadSelector) -> Self {
        Self::new_ext(&ctx.keyboard, &ctx.gamepads, selector)
    }

    /// Create a new input source.
    pub fn new_ext(keyboard: &Keyboard, gamepads: &Gamepads, selector: GamepadSelector) -> Self {
        Self(Rc::new(Inner {
            keyboard: keyboard.clone(),
            gamepads: gamepads.clone(),
            selector: RefCell::new(selector),
        }))
    }

    #[inline]
    pub(crate) fn keyboard(&self) -> &Keyboard {
        &self.0.keyboard
    }

    /// Create an input source that always listens to the most recently active gamepad.
    pub fn last_active(ctx: &Context) -> Self {
        Self::new(ctx, GamepadSelector::LastActive)
    }

    /// Create an input source that listens to a specific gamepad.
    pub fn specific(ctx: &Context, gamepad: impl Into<Option<Gamepad>>) -> Self {
        Self::new(ctx, GamepadSelector::Specific(gamepad.into()))
    }

    /// Set the input's gamepad source.
    pub fn set_selector(&self, selector: GamepadSelector) {
        self.0.selector.replace(selector);
    }

    /// Set the input's gamepad source to a specific gamepad.
    pub fn set_specific(&self, gamepad: impl Into<Option<Gamepad>>) {
        self.set_selector(GamepadSelector::Specific(gamepad.into()));
    }

    /// Set the input's gamepad source to always listen to the last active gamepad.
    pub fn set_last_active(&self) {
        self.0.selector.replace(GamepadSelector::LastActive);
    }

    /// Read a value from the source's gamepad. Usually you will not call this directly.
    pub fn read<R>(&self, f: impl FnOnce(&Gamepad) -> R) -> Option<R> {
        match self.0.selector.borrow().deref() {
            GamepadSelector::LastActive => self.0.gamepads.last_active().as_ref().map(f),
            GamepadSelector::Specific(pad) => pad.as_ref().map(f),
        }
    }
}
