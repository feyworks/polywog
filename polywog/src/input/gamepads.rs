use super::{Gamepad, GamepadAxis, GamepadButton, GamepadStatus};
use crate::core::Context;
use fnv::FnvHashMap;
use gilrs::{Event, EventType, GamepadId, Gilrs};
use smallvec::SmallVec;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::time::SystemTime;
use std::{cell::RefCell, rc::Rc};

/// Handle to the gamepads state.
///
/// This handle can be cloned and passed around freely to give objects access to gamepads.
#[derive(Clone)]
pub struct Gamepads(Rc<State>);

impl Debug for Gamepads {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Gamepads").finish_non_exhaustive()
    }
}

struct State {
    gilrs: Option<RefCell<Gilrs>>,
    gamepads: RefCell<FnvHashMap<GamepadId, Pad>>,
    last_active: Cell<SystemTime>,
}

struct Pad {
    pad: Gamepad,

    #[cfg(feature = "lua")]
    userdata: mlua::AnyUserData,
}

impl Gamepads {
    pub(crate) fn new() -> Self {
        Self(Rc::new(State {
            gilrs: Gilrs::new().ok().map(RefCell::new),
            gamepads: RefCell::new(FnvHashMap::default()),
            last_active: Cell::new(SystemTime::UNIX_EPOCH),
        }))
    }

    #[allow(unused_variables)]
    pub(crate) fn update(&self, ctx: &Context) {
        let Some(mut gilrs) = self.0.gilrs.as_ref().map(|g| g.borrow_mut()) else {
            return;
        };
        let mut gamepads = self.0.gamepads.borrow_mut();
        while let Some(Event {
            id, event, time, ..
        }) = gilrs.next_event()
        {
            self.0.last_active.set(time);

            match event {
                EventType::ButtonPressed(btn, _) => {
                    if let Ok(btn) = GamepadButton::try_from(btn) {
                        let pad = gamepads.get(&id).unwrap();
                        pad.pad.update_status(&gilrs, time);
                        pad.pad.handle_press(btn);
                    }
                }
                EventType::ButtonRepeated(btn, _) => {
                    if let Ok(btn) = GamepadButton::try_from(btn) {
                        let pad = gamepads.get(&id).unwrap();
                        pad.pad.update_status(&gilrs, time);
                        pad.pad.handle_repeat(btn);
                    }
                }
                EventType::ButtonReleased(btn, _) => {
                    if let Ok(btn) = GamepadButton::try_from(btn) {
                        let pad = gamepads.get(&id).unwrap();
                        pad.pad.update_status(&gilrs, time);
                        pad.pad.handle_release(btn);
                    }
                }
                EventType::ButtonChanged(btn, val, _) => {
                    if let Ok(btn) = GamepadButton::try_from(btn) {
                        let pad = gamepads.get(&id).unwrap();
                        pad.pad.update_status(&gilrs, time);
                        pad.pad.handle_button_change(btn, val);
                    }
                }
                EventType::AxisChanged(axis, val, _) => {
                    if let Ok(axis) = GamepadAxis::try_from(axis) {
                        let val = match axis {
                            GamepadAxis::LeftY | GamepadAxis::RightY | GamepadAxis::DPadY => -val,
                            _ => val,
                        };
                        let pad = gamepads.get(&id).unwrap();
                        pad.pad.update_status(&gilrs, time);
                        pad.pad.handle_axis_change(axis, val);
                    }
                }
                EventType::Connected => {
                    let pad = gilrs.connected_gamepad(id).unwrap();
                    let name = pad.name().to_string();
                    let status = GamepadStatus::from(pad.power_info());
                    let pad = Gamepad::new(id, name, status, time);
                    let pad = Pad {
                        #[cfg(feature = "lua")]
                        userdata: ctx.lua.upgrade().create_userdata(pad.clone()).unwrap(),
                        pad,
                    };
                    assert!(gamepads.insert(id, pad).is_none());
                }
                EventType::Disconnected => {
                    let pad = gamepads.remove(&id).unwrap();
                    pad.pad.disconnect();
                }
                EventType::Dropped => {}
                EventType::ForceFeedbackEffectCompleted => {}
                _ => {}
            }
        }
    }

    #[inline]
    pub(crate) fn clear_phase(&self) {
        for pad in self.0.gamepads.borrow().values() {
            pad.pad.clear_phase();
        }
    }

    #[inline]
    pub(crate) fn set_update_phase(&self) {
        for pad in self.0.gamepads.borrow().values() {
            pad.pad.set_update_phase();
        }
    }

    #[inline]
    pub(crate) fn set_render_phase(&self) {
        for pad in self.0.gamepads.borrow().values() {
            pad.pad.set_render_phase();
        }
    }

    /// How many gamepads are connected.
    #[inline]
    pub fn count(&self) -> usize {
        self.0.gamepads.borrow().len()
    }

    /// All connected gamepads.
    #[inline]
    pub fn all(&self) -> impl Iterator<Item = Gamepad> {
        self.0
            .gamepads
            .borrow()
            .values()
            .map(|pad| pad.pad.clone())
            .collect::<SmallVec<[_; 8]>>()
            .into_iter()
    }

    #[cfg(feature = "lua")]
    #[inline]
    pub fn all_lua(&self) -> impl Iterator<Item = mlua::AnyUserData> {
        self.0
            .gamepads
            .borrow()
            .values()
            .map(|pad| pad.userdata.clone())
            .collect::<SmallVec<[_; 8]>>()
            .into_iter()
    }

    /// All gamepads that connected this frame.
    #[inline]
    pub fn newly_connected(&self) -> impl Iterator<Item = Gamepad> {
        self.0
            .gamepads
            .borrow()
            .values()
            .filter(|pad| pad.pad.was_connected())
            .map(|pad| pad.pad.clone())
            .collect::<SmallVec<[_; 8]>>()
            .into_iter()
    }

    #[cfg(feature = "lua")]
    #[inline]
    pub(crate) fn newly_connected_lua(&self) -> impl Iterator<Item = mlua::AnyUserData> {
        self.0
            .gamepads
            .borrow()
            .values()
            .filter(|pad| pad.pad.was_connected())
            .map(|pad| pad.userdata.clone())
            .collect::<SmallVec<[_; 8]>>()
            .into_iter()
    }

    /// Last time any gamepad was updated.
    #[inline]
    pub fn last_active_time(&self) -> SystemTime {
        self.0.last_active.get()
    }

    /// The most recently active gamepad.
    #[inline]
    pub fn last_active(&self) -> Option<Gamepad> {
        self.0
            .gamepads
            .borrow()
            .values()
            .max_by_key(|pad| pad.pad.last_update())
            .map(|pad| pad.pad.clone())
    }

    #[cfg(feature = "lua")]
    #[inline]
    pub(crate) fn last_active_lua(&self) -> Option<mlua::AnyUserData> {
        self.0
            .gamepads
            .borrow()
            .values()
            .max_by_key(|pad| pad.pad.last_update())
            .map(|pad| pad.userdata.clone())
    }
}
