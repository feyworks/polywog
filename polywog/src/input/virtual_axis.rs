use super::VirtualButton;
use crate::input::GamepadAxis;
use crate::input::virtual_source::VirtualSource;
use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::rc::Rc;

/// Handle to a virtual axis.
///
/// This can be used to simultaneously listen to the state of a gamepad axis, but also to
/// treat a pair of buttons as negative and positive inputs for that axis. This handle can be
/// cloned and passed around to give objects access to it.
#[derive(Clone)]
pub struct VirtualAxis(Rc<Inner>);

impl PartialEq for VirtualAxis {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for VirtualAxis {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Rc::as_ptr(&self.0).partial_cmp(&Rc::as_ptr(&other.0))
    }
}

struct Inner {
    source: VirtualSource,
    axis: Cell<Option<GamepadAxis>>,
    neg: RefCell<VirtualButton>,
    pos: RefCell<VirtualButton>,
}

impl VirtualAxis {
    /// Create a new axis with no mappings.
    pub fn empty(source: &VirtualSource) -> Self {
        Self::new(source, None, None, None)
    }

    /// Create a new axis with the provided axis and negative/positive buttons.
    pub fn new(
        source: &VirtualSource,
        axis: impl Into<Option<GamepadAxis>>,
        neg: impl Into<Option<VirtualButton>>,
        pos: impl Into<Option<VirtualButton>>,
    ) -> Self {
        let neg = neg
            .into()
            .unwrap_or_else(|| VirtualButton::new(source, None, None));
        let pos = pos
            .into()
            .unwrap_or_else(|| VirtualButton::new(source, None, None));
        Self(Rc::new(Inner {
            source: source.clone(),
            axis: Cell::new(axis.into()),
            neg: RefCell::new(neg),
            pos: RefCell::new(pos),
        }))
    }

    /// The axis this input listens to.
    #[inline]
    pub fn axis(&self) -> Option<GamepadAxis> {
        self.0.axis.get()
    }

    /// The input's button on the negative axis.
    #[inline]
    pub fn neg(&self) -> VirtualButton {
        self.0.neg.borrow().clone()
    }

    /// The input's button on the positive axis.
    #[inline]
    pub fn pos(&self) -> VirtualButton {
        self.0.pos.borrow().clone()
    }

    /// Set the input's gamepad axis.
    pub fn set_axis(&self, axis: impl Into<Option<GamepadAxis>>) {
        self.0.axis.set(axis.into());
    }

    /// Set the input's negative and positive buttons.
    pub fn set_buttons(&self, neg: VirtualButton, pos: VirtualButton) {
        *self.0.neg.borrow_mut() = neg.into();
        *self.0.pos.borrow_mut() = pos.into();
    }

    /// If the axis state changed this frame.
    #[inline]
    pub fn changed(&self) -> bool {
        self.0
            .axis
            .get()
            .and_then(|axis| self.0.source.read(|pad| pad.axis_changed(axis)))
            .unwrap_or(false)
            || self.0.neg.borrow().changed()
            || self.0.pos.borrow().changed()
    }

    /// The axis value from `-1.0` to `1.0`.
    #[inline]
    pub fn value(&self) -> f32 {
        let mut value = 0.0;
        if let Some(axis) = self.0.axis.get() {
            value += self.0.source.read(|pad| pad.axis(axis)).unwrap_or(0.0);
        }
        value -= self.0.neg.borrow().value();
        value += self.0.pos.borrow().value();
        value.clamp(-1.0, 1.0)
    }
}
