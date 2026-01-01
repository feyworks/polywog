use crate::input::MouseButton;
use crate::math::{Numeric, Vec2F, vec2};
use dpi::{LogicalPosition, PhysicalPosition};
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::time::SystemTime;
use winit::event::{ElementState, MouseScrollDelta};

/// Handle to the mouse state.
///
/// This handle can be cloned and passed around freely to give objects access to the mouse.
#[derive(Clone)]
pub struct Mouse(Rc<State>);

impl Debug for Mouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mouse").finish_non_exhaustive()
    }
}

#[derive(Debug)]
struct State {
    pos: Cell<Vec2F>,
    down: Cell<[bool; 3]>,
    phases: [Phase; 2],
    phase: Cell<usize>,
    last_active: Cell<SystemTime>,
}

#[derive(Debug, Default)]
struct Phase {
    scroll_lines: Cell<Vec2F>,
    scroll_delta: Cell<Vec2F>,
    pressed: Cell<[bool; 3]>,
    released: Cell<[bool; 3]>,
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self(Rc::new(State {
            pos: Cell::new(Vec2F::ZERO),
            down: Cell::new([false; _]),
            phases: std::array::from_fn(|_| Phase::default()),
            phase: Cell::new(0),
            last_active: Cell::new(SystemTime::UNIX_EPOCH),
        }))
    }

    /// Time the mouse state last changed.
    #[inline]
    pub fn last_active(&self) -> SystemTime {
        self.0.last_active.get()
    }

    #[inline]
    fn phase(&self) -> &Phase {
        &self.0.phases[self.0.phase.get()]
    }

    /// The mouse position in window coordinates.
    #[inline]
    pub fn pos(&self) -> Vec2F {
        self.0.pos.get()
    }

    /// How many lines were scrolled this frame.
    #[inline]
    pub fn scroll_lines(&self) -> Vec2F {
        self.phase().scroll_lines.get()
    }

    /// Delta vector of the mouse scroll this frame.
    #[inline]
    pub fn scroll_delta(&self) -> Vec2F {
        self.phase().scroll_delta.get()
    }

    /// If the button is down.
    #[inline]
    pub fn down(&self, button: MouseButton) -> bool {
        Cell::as_array_of_cells(&self.0.down)[button as usize].get()
    }

    /// If the left button is down.
    #[inline]
    pub fn left_down(&self) -> bool {
        self.down(MouseButton::Left)
    }

    /// If the middle button is down.
    #[inline]
    pub fn middle_down(&self) -> bool {
        self.down(MouseButton::Middle)
    }

    /// If the right button is down.
    #[inline]
    pub fn right_down(&self) -> bool {
        self.down(MouseButton::Right)
    }

    /// If the button was pressed this frame.
    #[inline]
    pub fn pressed(&self, button: MouseButton) -> bool {
        Cell::as_array_of_cells(&self.phase().pressed)[button as usize].get()
    }

    /// If the left button was pressed this frame.
    #[inline]
    pub fn left_pressed(&self) -> bool {
        self.pressed(MouseButton::Left)
    }

    /// If the middle button was pressed this frame.
    #[inline]
    pub fn middle_pressed(&self) -> bool {
        self.pressed(MouseButton::Middle)
    }

    /// If the right button was pressed this frame.
    #[inline]
    pub fn right_pressed(&self) -> bool {
        self.pressed(MouseButton::Right)
    }

    /// If the button was released this frame.
    #[inline]
    pub fn released(&self, button: MouseButton) -> bool {
        Cell::as_array_of_cells(&self.phase().released)[button as usize].get()
    }

    /// If the left button was released this frame.
    #[inline]
    pub fn left_released(&self) -> bool {
        self.released(MouseButton::Left)
    }

    /// If the middle button was released this frame.
    #[inline]
    pub fn middle_released(&self) -> bool {
        self.released(MouseButton::Middle)
    }

    /// If the right button was released this frame.
    #[inline]
    pub fn right_released(&self) -> bool {
        self.released(MouseButton::Right)
    }

    #[inline]
    pub(crate) fn set_update_phase(&self) {
        self.0.phase.set(0);
    }

    #[inline]
    pub(crate) fn set_render_phase(&self) {
        self.0.phase.set(1);
    }

    #[inline]
    pub(crate) fn handle_move(&self, pos: LogicalPosition<f32>) {
        self.0.last_active.set(SystemTime::now());

        self.0.pos.set(vec2(pos.x, pos.y));
    }

    #[inline]
    pub(crate) fn handle_scroll(&self, delta: MouseScrollDelta) {
        self.0.last_active.set(SystemTime::now());

        let phase = &self.0.phases[self.0.phase.get()];
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                phase.scroll_lines.update(|s| s + vec2(x, y));
            }
            MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
                phase.scroll_delta.update(|s| s + vec2(x, y).to_f32());
            }
        }
    }

    #[inline]
    pub(crate) fn handle_input(&self, button: winit::event::MouseButton, state: ElementState) {
        self.0.last_active.set(SystemTime::now());

        let Ok(button) = MouseButton::try_from(button) else {
            return;
        };
        let button = button as usize;
        match state {
            ElementState::Pressed => {
                Cell::as_array_of_cells(&self.0.down)[button].set(true);
                for phase in &self.0.phases {
                    Cell::as_array_of_cells(&phase.pressed)[button].set(true);
                }
            }
            ElementState::Released => {
                Cell::as_array_of_cells(&self.0.down)[button].set(false);
                for phase in &self.0.phases {
                    Cell::as_array_of_cells(&phase.released)[button].set(true);
                }
            }
        }
    }

    #[inline]
    pub(crate) fn clear_phase(&self) {
        let phase = self.phase();
        phase.scroll_lines.set(Vec2F::ZERO);
        phase.scroll_delta.set(Vec2F::ZERO);
        phase.pressed.set([false; 3]);
        phase.released.set([false; 3]);
    }
}
