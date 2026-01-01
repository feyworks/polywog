use crate::input::{GamepadButton, GamepadStatus};
use gilrs::{GamepadId, Gilrs};
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::{rc::Rc, time::SystemTime};
use strum::EnumCount;

use super::GamepadAxis;

/// Handle to a gamepad.
///
/// This handle can be cloned and passed around freely to give objects access to the gamepad.
#[derive(Clone)]
pub struct Gamepad(Rc<State>);

impl Debug for Gamepad {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Gamepad").finish_non_exhaustive()
    }
}

impl PartialEq for Gamepad {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialOrd for Gamepad {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Rc::as_ptr(&self.0).partial_cmp(&Rc::as_ptr(&other.0))
    }
}

struct State {
    id: GamepadId,
    name: String,
    status: Cell<GamepadStatus>,
    down: Cell<[bool; GamepadButton::COUNT]>,
    btn_value: Cell<[f32; GamepadButton::COUNT]>,
    axis_value: Cell<[f32; GamepadAxis::COUNT]>,
    phases: [Phase; 2],
    phase: Cell<usize>,
    last_update: Cell<SystemTime>,
    connect_time: SystemTime,
    connected: Cell<bool>,
}

#[derive(Clone)]
struct Phase {
    was_connected: Cell<bool>,
    pressed: Cell<[bool; GamepadButton::COUNT]>,
    released: Cell<[bool; GamepadButton::COUNT]>,
    repeated: Cell<[bool; GamepadButton::COUNT]>,
    btn_changed: Cell<[bool; GamepadButton::COUNT]>,
    axis_changed: Cell<[bool; GamepadAxis::COUNT]>,
}

impl Default for Phase {
    fn default() -> Self {
        Self {
            was_connected: Cell::new(false),
            pressed: Cell::new([false; _]),
            released: Cell::new([false; _]),
            repeated: Cell::new([false; _]),
            btn_changed: Cell::new([false; _]),
            axis_changed: Cell::new([false; _]),
        }
    }
}

impl Gamepad {
    pub(crate) fn new(
        id: GamepadId,
        name: String,
        status: GamepadStatus,
        connect_time: SystemTime,
    ) -> Self {
        Self(Rc::new(State {
            id,
            name,
            status: Cell::new(status),
            down: Cell::new([false; _]),
            btn_value: Cell::new([0.0; _]),
            axis_value: Cell::new([0.0; _]),
            phases: std::array::repeat(Phase::default()),
            phase: Cell::new(0),
            last_update: Cell::new(connect_time),
            connect_time,
            connected: Cell::new(true),
        }))
    }

    pub(crate) fn disconnect(&self) {
        self.0.connected.set(false);
        self.0.status.set(GamepadStatus::Unknown);
        self.0.down.set([false; _]);
        self.0.btn_value.set([0.0; _]);
        self.0.axis_value.set([0.0; _]);
        for phase in &self.0.phases {
            phase.was_connected.set(false);
            phase.pressed.set([false; _]);
            phase.repeated.set([false; _]);
            phase.released.set([false; _]);
            phase.btn_changed.set([false; _]);
            phase.axis_changed.set([false; _]);
        }
    }

    #[inline]
    fn phase(&self) -> &Phase {
        &self.0.phases[self.0.phase.get()]
    }

    /// The gamepad name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.0.name
    }

    /// If the gamepad was connected this frame.
    #[inline]
    pub fn was_connected(&self) -> bool {
        self.0.phases[self.0.phase.get()].was_connected.get()
    }

    /// Time the gamepad was connected.
    #[inline]
    pub fn connect_time(&self) -> SystemTime {
        self.0.connect_time
    }

    /// Charging status of the gamepad.
    #[inline]
    pub fn charging_status(&self) -> GamepadStatus {
        self.0.status.get()
    }

    /// Time the gamepad was last updated.
    #[inline]
    pub fn last_update(&self) -> SystemTime {
        self.0.last_update.get()
    }

    /// If the button is down.
    #[inline]
    pub fn down(&self, btn: GamepadButton) -> bool {
        Cell::as_array_of_cells(&self.0.down)[btn as usize].get()
    }

    /// If the button was pressed this frame.
    #[inline]
    pub fn pressed(&self, btn: GamepadButton) -> bool {
        Cell::as_array_of_cells(&self.phase().pressed)[btn as usize].get()
    }

    /// If the button was released this frame.
    #[inline]
    pub fn released(&self, btn: GamepadButton) -> bool {
        Cell::as_array_of_cells(&self.phase().released)[btn as usize].get()
    }

    /// If the button was repeated this frame.
    #[inline]
    pub fn repeated(&self, btn: GamepadButton) -> bool {
        Cell::as_array_of_cells(&self.phase().repeated)[btn as usize].get()
    }

    /// If the button state changed this frame.
    #[inline]
    pub fn btn_changed(&self, btn: GamepadButton) -> bool {
        Cell::as_array_of_cells(&self.phase().btn_changed)[btn as usize].get()
    }

    /// The button value from `0.0` (fully up) to `1.0` (fully down).
    #[inline]
    pub fn value(&self, btn: GamepadButton) -> f32 {
        Cell::as_array_of_cells(&self.0.btn_value)[btn as usize].get()
    }

    /// The axis value from `-1.0` to `1.0`.
    #[inline]
    pub fn axis(&self, axis: GamepadAxis) -> f32 {
        Cell::as_array_of_cells(&self.0.axis_value)[axis as usize].get()
    }

    /// If the axis changed this frame.
    #[inline]
    pub fn axis_changed(&self, axis: GamepadAxis) -> bool {
        Cell::as_array_of_cells(&self.phase().axis_changed)[axis as usize].get()
    }

    #[inline]
    pub(crate) fn update_status(&self, gilrs: &Gilrs, time: SystemTime) {
        let power_info = gilrs.connected_gamepad(self.0.id).unwrap().power_info();
        self.0.status.set(GamepadStatus::from(power_info));
        self.0.last_update.set(time);
    }

    pub(crate) fn handle_press(&self, btn: GamepadButton) {
        let btn = btn as usize;
        Cell::as_array_of_cells(&self.0.down)[btn].set(true);
        for phase in &self.0.phases {
            Cell::as_array_of_cells(&phase.pressed)[btn].set(true);
        }
    }

    pub(crate) fn handle_repeat(&self, btn: GamepadButton) {
        let btn = btn as usize;
        for phase in &self.0.phases {
            Cell::as_array_of_cells(&phase.repeated)[btn].set(true);
        }
    }

    pub(crate) fn handle_release(&self, btn: GamepadButton) {
        let btn = btn as usize;
        Cell::as_array_of_cells(&self.0.down)[btn].set(false);
        for phase in &self.0.phases {
            Cell::as_array_of_cells(&phase.released)[btn].set(true);
        }
    }

    pub(crate) fn handle_button_change(&self, btn: GamepadButton, val: f32) {
        Cell::as_array_of_cells(&self.0.btn_value)[btn as usize].set(val);
    }

    pub(crate) fn handle_axis_change(&self, axis: GamepadAxis, val: f32) {
        Cell::as_array_of_cells(&self.0.axis_value)[axis as usize].set(val);
    }

    pub(crate) fn set_update_phase(&self) {
        self.0.phase.set(0);
    }

    pub(crate) fn set_render_phase(&self) {
        self.0.phase.set(1);
    }

    pub(crate) fn clear_phase(&self) {
        let phase = self.phase();
        phase.was_connected.set(false);
        phase.pressed.set([false; _]);
        phase.repeated.set([false; _]);
        phase.released.set([false; _]);
        phase.btn_changed.set([false; _]);
        phase.axis_changed.set([false; _]);
    }
}
