use crate::input::virtual_source::VirtualSource;
use crate::input::{GamepadButton, Key, Keyboard};
use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

/// Handle to a virtual button.
///
/// This can be used to simultaneously listen to the state of a key and gamepad button, and
/// can be cloned and passed around to give objects access to it.
#[derive(Clone)]
pub struct VirtualButton(Rc<Inner>);

impl PartialEq for VirtualButton {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for VirtualButton {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Rc::as_ptr(&self.0).partial_cmp(&Rc::as_ptr(&other.0))
    }
}

struct Inner {
    source: VirtualSource,
    btn: Cell<Option<GamepadButton>>,
    key: Cell<Option<Key>>,
}

impl VirtualButton {
    /// Create a button with no mappings.
    pub fn empty(source: &VirtualSource) -> Self {
        Self::new(source, None, None)
    }

    /// Create a button that listens to the provided key and button.
    pub fn new(
        source: &VirtualSource,
        key: impl Into<Option<Key>>,
        btn: impl Into<Option<GamepadButton>>,
    ) -> Self {
        Self(Rc::new(Inner {
            source: source.clone(),
            btn: Cell::new(btn.into()),
            key: Cell::new(key.into()),
        }))
    }

    #[inline]
    fn keyboard(&self) -> &Keyboard {
        self.0.source.keyboard()
    }

    /// Set the gamepad button to listen to.
    #[inline]
    pub fn set_button(&self, btn: impl Into<Option<GamepadButton>>) {
        self.0.btn.set(btn.into());
    }

    /// Set the key to listen to.
    #[inline]
    pub fn set_key(&self, key: impl Into<Option<Key>>) {
        self.0.key.set(key.into());
    }

    /// If the button's key or gamepad button is down.
    #[inline]
    pub fn down(&self) -> bool {
        self.0
            .btn
            .get()
            .is_some_and(|btn| self.0.source.read(|pad| pad.down(btn)).unwrap_or(false))
            || self
                .0
                .key
                .get()
                .is_some_and(|key| self.keyboard().down(key))
    }

    /// If the button's key or gamepad button was pressed this frame.
    ///
    /// This treats the mappings as if they were one button, meaning if the gamepad button is held
    /// down and you press the key, it will not count as a press because the virtual button is
    /// already considered down.
    #[inline]
    pub fn pressed(&self) -> bool {
        let mut pressed = false;
        if let Some(btn) = self.0.btn.get() {
            if let Some((press, down)) = self.0.source.read(|pad| (pad.pressed(btn), pad.down(btn)))
            {
                if press {
                    pressed = true;
                } else if down {
                    return false;
                }
            }
        }
        if let Some(key) = self.0.key.get() {
            if self.keyboard().pressed(key) {
                pressed = true;
            } else if self.keyboard().down(key) {
                return false;
            }
        }
        pressed
    }

    /// If the button's key or gamepad button was pressed this frame.
    ///
    /// This treats the mappings as if they were one button, meaning if the gamepad button is
    /// released but the key is still held down, this will not return true until the key is also
    /// released.
    #[inline]
    pub fn released(&self) -> bool {
        let mut released = false;
        if let Some(btn) = self.0.btn.get() {
            if let Some((release, down)) =
                self.0.source.read(|pad| (pad.released(btn), pad.down(btn)))
            {
                if release {
                    released = true;
                } else if !down {
                    return false;
                }
            }
        }
        if let Some(key) = self.0.key.get() {
            if self.keyboard().released(key) {
                released = true;
            } else if !self.keyboard().down(key) {
                return false;
            }
        }
        released
    }

    /// If the button state changed this frame.
    #[inline]
    pub fn changed(&self) -> bool {
        self.0.btn.get().is_some_and(|btn| {
            self.0
                .source
                .read(|pad| pad.btn_changed(btn))
                .unwrap_or(false)
        }) || self
            .0
            .key
            .get()
            .is_some_and(|key| self.keyboard().pressed(key) || self.keyboard().released(key))
    }

    /// Value of the button.
    #[inline]
    pub fn value(&self) -> f32 {
        if self
            .0
            .key
            .get()
            .is_some_and(|key| self.keyboard().down(key))
        {
            return 1.0;
        }
        self.0
            .btn
            .get()
            .and_then(|btn| self.0.source.read(|pad| pad.value(btn)))
            .unwrap_or(0.0)
    }
}
