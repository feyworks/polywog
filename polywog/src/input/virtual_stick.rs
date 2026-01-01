use super::VirtualAxis;
use crate::input::virtual_source::VirtualSource;
use crate::math::{Vec2, vec2};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Handle to a virtual stick.
///
/// This combines a virtual x and y axis to simulate a thumbstick, and can be cloned and passed
/// around to give objects access to it.
#[derive(Clone)]
pub struct VirtualStick(Rc<Inner>);

impl Debug for VirtualStick {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("VirtualStick").finish_non_exhaustive()
    }
}

impl PartialEq for VirtualStick {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for VirtualStick {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Rc::as_ptr(&self.0).partial_cmp(&Rc::as_ptr(&other.0))
    }
}

struct Inner {
    pub x_axis: RefCell<VirtualAxis>,
    pub y_axis: RefCell<VirtualAxis>,
}

impl VirtualStick {
    /// Create a new stick with no mappings.
    pub fn empty(source: &VirtualSource) -> Self {
        Self::new(source, None, None)
    }

    /// Create a new stick with the provided x and y axes.
    pub fn new(
        source: &VirtualSource,
        x_axis: impl Into<Option<VirtualAxis>>,
        y_axis: impl Into<Option<VirtualAxis>>,
    ) -> Self {
        let x_axis = x_axis.into().unwrap_or_else(|| VirtualAxis::empty(source));
        let y_axis = y_axis.into().unwrap_or_else(|| VirtualAxis::empty(source));
        Self(Rc::new(Inner {
            x_axis: RefCell::new(x_axis),
            y_axis: RefCell::new(y_axis),
        }))
    }

    /// The stick's x-axis.
    #[inline]
    pub fn x_axis(&self) -> VirtualAxis {
        self.0.x_axis.borrow().clone()
    }

    /// The stick's y-axis.
    #[inline]
    pub fn y_axis(&self) -> VirtualAxis {
        self.0.y_axis.borrow().clone()
    }

    /// Set the stick's axes.
    pub fn set_axes(&self, x_axis: VirtualAxis, y_axis: VirtualAxis) {
        *self.0.x_axis.borrow_mut() = x_axis;
        *self.0.y_axis.borrow_mut() = y_axis;
    }

    /// If either axes changed this frame.
    #[inline]
    pub fn changed(&self) -> bool {
        self.0.x_axis.borrow().changed() || self.0.y_axis.borrow().changed()
    }

    /// The stick's non-normalized x-value.
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.x_axis.borrow().value()
    }

    /// The stick's non-normalized y-value.
    #[inline]
    pub fn y(&self) -> f32 {
        self.0.y_axis.borrow().value()
    }

    /// The stick's normalized value.
    #[inline]
    pub fn value(&self) -> Vec2<f32> {
        let value = vec2(self.x(), self.y());
        let len = value.len();
        if len > 1.0 { value / len } else { value }
    }
}
